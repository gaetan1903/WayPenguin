# WayPenguin — Algorithmes

## Machine à états (PetState)

```
Idle ──(timeout)──> Sleep
 Idle ──(cooldown)──> LookAround
 Idle ──(cursor)──> Walk/Run/Stomp/Climb
Wander ──(timer)──> Idle
Walk/Run ──(destination)──> Idle
  Float ──(timer 3-8s)──> Fall
 Stomp ──(descent)──> Land
 Climb ──(descent)──> Land
  Fall ──(sol touché)──> Land
  Land ──(1 frame)──> Idle
Tumble ──(timer)──> Idle
Squish ──(frames)──> Splat ──(timer)──> Angel ──(timer)──> Idle
```

### Transitions clés
- **Idle → Walk/Run/Stomp/Climb** : déclenché par le survol du curseur sur le pet. Si curseur est très rapide (>20px/frame) → Run. Si pet est sur un bord de l'écran → retour vers l'intérieur.
- **Idle → Wander** : quand le pet décide d'explorer tout seul. Timer aléatoire (5-20s d'idle). Cible choisie pour maximiser la distance aux autres pets.
- **Idle → Float** : le pet se met à flotter vers le haut. Float auto-limité (3-8s) puis retombe.
- **Climb/Stomp** : disponibles partout, pas seulement aux bords de l'écran. Climb monte, Stomp descend.

---

## Anti-regroupement (3 mécanismes)

### 1. Wander target intelligent
Quand un pet choisit une cible Wander, il ne prend pas une position aléatoire simple. Il génère 6 candidats aléatoires et garde celui qui maximise `min_distance_to_other_pets`.

Pseudo-code :
```
best_score = 0
for _ in 0..6:
    candidate = random_point_on_screen()
    min_d = +inf
    for other in others:
        d = distance(candidate, other)
        min_d = min(min_d, d)
    if min_d > best_score:
        best_score = min_d
        best_candidate = candidate
target = best_candidate
```

Résultat : les pets wander vers des zones naturellement éloignées les unes des autres.

### 2. Force d'évitement renforcée
Appliquée en continu dans `apply_avoidance()` sur chaque paire de pets :

- Si distance < AVOID_DIST (120px) : pousse la cible de déplacement + la position directe
- Force = 6× (× 12× si très proche, distance < 40px)
- Push direct `self.x/y += avoid * 0.1` : les pets s'écartent immédiatement, pas seulement leur cible

### 3. Dérive idle
Quand un pet est Idle et qu'un autre pet est à moins de 150px :

- Calcule le vecteur moyen d'éloignement (direction opposée)
- Si la somme des distances dépasse un seuil (500 px²) : fixe une cible de dérive douce (`speed = 1.0`)
- Le pet s'éloigne lentement même en idle, sans changer d'état

---

## Physique et mouvement

- `grounded = true` quand `y >= floor_y` (sol = `screen_h - 130`)
- `vy += GRAVITY` (0.4 px/frame²) quand en l'air
- `speed = 2.0` idle/wander, `speed = 4.0` walk, `speed = 8.0` run
- Float annule la gravité et monte régulièrement jusqu'à `floor_y * 0.5` max
- Les collisions avec les bords de l'écran sont réactives (demi-tour après 1s vs 2s avant)
- **Décélération douce** : quand un pet approche de sa cible (dist < 30px), la vitesse est multipliée par `dist / 30`, créant un ease-out naturel. Le seuil d'arrêt est passé de 5px à 0.5px pour une précision meilleure.
- `self.width` remplace le `64.0` hardcodé pour les calculs de bordure — la taille logique du pet est configurable.

---

## Archetypes

Chaque pet a un archétype qui définit ses probabilités de comportement :

| Nom             | walk | run  | float | wander | sleep_timeout |
|-----------------|------|------|-------|--------|---------------|
| Explorer        | 0.3  | 0.2  | 0.05  | 0.3    | 120s          |
| Floater         | 0.1  | 0.1  | 0.3   | 0.1    | 90s           |
| CouchPotato     | 0.05 | 0.02 | 0.05  | 0.05   | 300s          |
| Hyperactive     | 0.1  | 0.4  | 0.1   | 0.4    | 45s           |
| Jumper          | 0.3  | 0.3  | 0.2   | 0.2    | 60s           |

`pick_idle_behavior()` = tirage pondéré au moment du choix (pas de timer par comportement).

---

## Rendu et chaîne graphique

### Pipeline actuel (SHM, CPU)

1. **Thème XPM** → pixels ARGB 30×30 en mémoire
2. **nearest_neighbor_scale()** ×3 (30→90 px) — pixel doubling parfait, pas de flou
3. **Buffer final** 90×90 px — plus aucune bordure transparente (le centrage est un no-op car frame_w == PET_SIZE)
4. **Ombre portée** : ellipse douce rendue dans le buffer avant le sprite
   - Largeur : 55% du buffer (idle), 70% (run)
   - Opacité : 55% au sol, fond à mesure que le pet monte (hauteur / 300px)
   - Stretch horizontal quand le pet court
5. **Respiration** : oscillation sinusoïdale de ±1px sur l'axe Y (période 2.2s) appliquée via `composite_frame` avec offset. Actif seulement en Idle, LookAround, Sleep.
6. **Alpha blending ARGB** : `alpha_blend()` avec calcul correct des canaux — permet la composition ombre + sprite sans artefact
7. **Copie → SHM buffer → layer-shell window**

### PET_SIZE = 90
- 3× integer exact des frames thème (30×30) → 90×90 px nets
- 50% plus grand qu'avant (60→90), plus lisible sur écran haute résolution
- Fenêtre ≈ 0.63 po sur un 34" 3440×1440 (vs 0.42 po avant)

### Alpha blending
`alpha_blend(src, dst)` effectue un blending correct (non prémultiplié) :
- Si alpha source = 0 → retourne dst inchangé
- Si alpha source ≥ 254 → retourne src (opaque)
- Sinon : calcule alpha résultat = sa + da * (1 - sa) / 255, puis chaque canal pondéré

### Ombres
- `render_contact_shadow()` : ellipse floutée sous les pieds du pet
- `render_ambient_shadow()` : halo ombré autour du personnage (désactivé par défaut, prêt à l'emploi)
- Les ombres utilisent `alpha_blend()` pour se fondre proprement

### Animation respiratoire
- Onde sinusoïdale : `sin(elapsed_ms / 2200 * 2π) * 1.0`
- Période ≈ 2.2 secondes (respiration naturelle)
- Amplitude 1px (subtile, pas distrayante)
- Appliquée en Idle, Sleep, LookAround

### Décélération visuelle
- `ease_dist = 30px` : quand le pet arrive à moins de 30px de sa cible, sa vitesse effective est multipliée par `dist / ease_dist`
- Résultat : le pet ralentit progressivement au lieu de s'arrêter net
- Le seuil de précision est de 0.5px (au lieu de 5px avant)
- Combiné avec la respiration, l'arrêt du pet est naturel et organique

---

## Architecture du rendu (waypenguin-renderer)

### Fonctions publiques
| Fonction | Description |
|----------|-------------|
| `render_frame()` | Nettoie le buffer + dessine le sprite (compat ascendante) |
| `composite_frame()` | Dessine le sprite avec offset x/y, sans nettoyer |
| `render_contact_shadow()` | Ombre portée elliptique avec fondu |
| `render_ambient_shadow()` | Halo d'ambiance autour du sprite |
| `breathing_offset()` | Décalage Y sinusoïdal pour la respiration |
| `squash_stretch()` | Facteurs d'échelle basés sur vitesse/vy |

### Prévu (non implémenté)
- Rendu GPU via wayland-egl / OpenGL
- Shaders de glow, flou, effets saisonniers
- Feuilles de sprite HD multi-résolution
- Interpolation temporelle entre frames d'animation

### Fallback procédural
Si le thème Penguins n'est pas trouvé, un Tux procédural est généré dans `waypenguin-assets`. Le rendu utilise alors `composite_frame` avec un buffer source 64×64 → redimensionné par le renderer dans le buffer 90×90.
