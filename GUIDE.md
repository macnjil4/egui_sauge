# Guide UX/UI — egui_sauge

Ce document décrit comment composer une bonne IHM avec le design system. C'est un complément à la doc API (`cargo doc --open`) : l'API dit *quoi* exposer, ce guide dit *quand* et *comment*.

> **TL;DR** Topbar fine, navigation persistante à gauche, contenu au centre. Les actions destructives utilisent `Button::danger` + `ConfirmDialog`. Le bouton primaire est à droite. Modal pour les actions bloquantes, side panel pour les paramètres révocables. Une seule action primaire par écran.

---

## 1. Anatomie d'une page

```
┌─────────────────────────────────────────────────────┐
│  TopBar  · brand · search · user menu               │  ← egui::Panel::top
├──────────┬──────────────────────────────────────────┤
│          │  PageHeader (title, breadcrumb, actions) │
│ Sidebar  ├──────────────────────────────────────────┤
│ NavItems │                                          │
│          │  Tabs (optional)                         │
│          ├──────────────────────────────────────────┤
│          │                                          │
│          │  Content                                 │
│          │  - Cards · Stats · Tables                │
│          │                                          │
│          │                                          │
└──────────┴──────────────────────────────────────────┘
```

**Composants par zone**

| Zone | Composants à utiliser |
|---|---|
| Top bar | `Panel::top("topbar")` — logo (`Icon::Leaf`), recherche globale (`InputField` + `Icon::Search`), `Kbd("⌘K")`, avatar `IconButton::new(Icon::UserCircle)` |
| Sidebar | `egui::SidePanel::left` + `NavItem` (un par route, marquer `selected`) |
| Header de page | `PageHeader::new(...).breadcrumb(...).subtitle(...)` |
| Onglets | `Tabs::new(&mut state).tab(...)` directement sous le header |
| Cartes | `Card` pour chaque bloc thématique ; `Stat` pour les KPIs ; `KeyValue` pour les listes de propriétés |
| Toasts | un `Toasts` global, instancié dans l'état de l'app, affiché par-dessus tout via `.show(ctx)` à la fin de chaque frame |

**Densité.** Activer `Density::Compact` quand on liste >50 lignes (logs, tables, audit) ; rester en `Comfortable` partout ailleurs.

---

## 2. Navigation : choisir le bon pattern

| Pattern | Quand l'utiliser | Composant |
|---|---|---|
| **Sidebar** | 3-12 sections persistantes, accessibles à tout moment (Dashboard, Servers, Users, Settings…) | `NavItem` dans un `SidePanel::left` |
| **Top tabs** | Sous-vues d'une même entité (Détails / Logs / Permissions / Activité d'un serveur) | `Tabs` |
| **Breadcrumb** | Hiérarchie réelle de ressources (`Org › Project › Env › Service`). Toujours sous le header. | `Breadcrumb` ou `PageHeader::breadcrumb` |
| **Top nav (links)** | Apps simples sans sidebar (≤ 5 sections). | `Tabs` ou `Button::ghost` dans la topbar |
| **Drawer** | Filtres avancés, configuration ponctuelle révocable. | `egui::SidePanel::right` |
| **Modal** | Action bloquante, action destructive, action courte avec validation | `Dialog` / `ConfirmDialog` |

**Règles**
- Une seule action primaire par écran (la plus à droite du `PageHeader`).
- Les onglets ne nichent pas (jamais de tabs dans des tabs). Si vous y pensez, c'est une autre page.
- Le breadcrumb reflète l'URL/route, pas l'historique de navigation.

---

## 3. Side panel vs Modal — la décision

C'est la question la plus fréquente. Voici le critère unique :

> **Modal = bloquant. Side panel = révocable.**

| Choisir | Si… | Exemples |
|---|---|---|
| **Modal (`Dialog`)** | l'utilisateur **doit** confirmer ou annuler avant de faire autre chose. Action irréversible ou critique. Formulaire court (< 6 champs). | "Supprimer ce projet ?", "Saisir le code 2FA", "Renommer la branche" |
| **Side panel** (`SidePanel::right`) | les paramètres restent éditables en arrière-plan. L'utilisateur peut continuer à voir / sélectionner d'autres éléments. | Détails d'un serveur dans une liste, Filtres d'une table, Préférences d'affichage |
| **Inline** (`Card`) | la donnée est consultative ou éditée rarement. Pas de friction. | Profil utilisateur, page Settings classique |

**Anti-patterns**
- Modal pour un formulaire de >10 champs → utiliser une page dédiée.
- Side panel pour confirmer une suppression → utiliser `ConfirmDialog::new(...).danger()`.
- Deux modals empilés → refaire le flow, jamais empiler.
- Modal pour des actions purement informatives ("Voici la doc") → `Alert` ou `Tooltip`.

---

## 4. Ordre des boutons

L'ordre dépend de la plateforme cible. **Choisir une convention et s'y tenir.**

### Convention egui_sauge (par défaut)

L'action **primaire à droite**, suivie des actions secondaires, **annuler à gauche**. C'est la convention web / Material / Linear / Vercel — ce que la plupart des utilisateurs IT attendent dans un outil multi-plateforme.

```rust
ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
    if ui.add(Button::primary("Save")).clicked()      { /* … */ }   // ← right-most
    ui.add_space(SPACING.s2);
    if ui.add(Button::secondary("Cancel")).clicked()  { /* … */ }
});
```

C'est exactement ce que fait `Dialog::show` et `ConfirmDialog::show` : la zone d'actions est en `right_to_left`, le premier bouton ajouté apparaît à droite.

### macOS-natif (si l'app cible exclusivement macOS)

Inverser : **annuler à gauche, valider à droite, sans espacement plus large**, et utiliser le wording macOS ("OK" plutôt que "Confirm"). Pas de cas pour l'instant — `egui_sauge` suit la convention web.

### Hiérarchie visuelle

| Rôle | Variante |
|---|---|
| Action principale unique | `Button::primary` |
| Action secondaire | `Button::secondary` |
| Action annulable / faible emphase | `Button::ghost` |
| Action destructive | `Button::danger` |

**Une seule `primary` par contexte** (form, dialog, toolbar). Si vous avez deux actions équivalentes, les deux doivent être `secondary` ; aucune ne doit être primaire.

### Confirmation

Toute action qui ne peut **pas** se défaire par un Ctrl-Z évident demande un `ConfirmDialog` :

```rust
match ConfirmDialog::new("Supprimer ce projet ?",
    "Cette action est définitive. Toutes les ressources associées \
     seront supprimées dans 30 jours.")
    .danger()
    .confirm_label("Supprimer")
    .show(ctx)
{
    Some(true)  => { /* delete */ }
    Some(false) => { /* close */ }
    None        => { /* still open */ }
}
```

---

## 5. Hiérarchie typo & espacement

| Niveau | Style | Quand |
|---|---|---|
| Titre d'écran | `display` (40 px) | Splash, dashboard hero. Rare. |
| Titre de page | Body de `PageHeader` (28 px) | Titre principal en haut de chaque page. |
| Section | `Heading` (28 px) | Section dans un long flux (`Section::new(...)`). |
| Sous-section / titre de carte | `h3` (16 px, semibold) | `Card::new().title(...)`. |
| Texte courant | `Body` (14 px) | Tout le reste. |
| Méta / labels | `Small` (12 px) | Labels d'inputs, dates, secondary info. |
| Code, IDs, hex, durées | `Monospace` (13 px) | Toujours mono pour les choses qu'on copie/colle. |

**Règle d'or** : ne mélangez jamais deux tailles à 2 px d'écart (e.g. h3 16 et body-lg 16 sur la même ligne). Si deux infos doivent coexister, utilisez la couleur (`text_secondary`) plutôt qu'une autre taille.

**Espacement** : `SPACING.s2` (8 px) entre éléments d'une même unité ; `SPACING.s3-4` (12-16 px) entre éléments groupés ; `SPACING.s5` (24 px) entre sections.

---

## 6. Feedback : alert vs toast vs banner

| Pattern | Durée | Pour |
|---|---|---|
| `Alert` (inline) | persistant tant que la cause existe | "Disk usage > 80%", "Migration en cours" — rattaché à la zone concernée |
| `Toasts` (top-right, auto-dismiss) | 4-6 s | Confirmation d'une action ponctuelle ("Pull request créée"), erreur transitoire |
| `Banner` (full width, top de page) | persistant, dismissible | Maintenance planifiée, billing overdue — concerne toute l'app |

**Niveaux** : Info (blue), Success (green), Warning (amber), Error (red).
**Toujours** un icône en plus de la couleur (accessibilité). C'est ce que font `Alert` et `Toast` automatiquement via `Level::icon()`.

---

## 7. Forms : règles de base

- Une seule colonne par défaut. Deux colonnes acceptable seulement pour des paires logiques (city/zip, first/last).
- Label **au-dessus** du champ (jamais à gauche pour des UIs IT modernes : c'est plus rapide à scanner).
- Required ≠ visible : marquer ce qui est *optionnel* avec `(optional)` plutôt que coller une étoile partout.
- Erreurs sous le champ, en `palette.error`, avec un contenu actionnable. ❌ "Invalid". ✅ "Use your work email (@company.com)".
- Disabled n'est pas un état d'erreur : il faut une raison contextualisée (tooltip, helper).
- Mot de passe : `InputField::new(...).password(true)`. Toujours offrir l'œil pour révéler (`Icon::Eye` / `Icon::EyeSlash` en `trailing`).

---

## 8. Tables et listes

Pas encore de composant `Table` natif (todo v0.2). Pour l'instant, composer avec `egui::Grid` ou `egui::ScrollArea` + lignes en `Card::new().elevation(Elevation::Flat)`.

**Règles si vous le construisez vous-même** :
- Header sticky, fond `bg_surface_alt`.
- Cellules numériques alignées à droite, mono.
- IDs (UUID, hash, IP) toujours mono + tronqués avec ellipsis ; valeur complète au hover (`tooltip`).
- Lignes zebra optionnelles (utiliser `bg_surface_alt` 1 ligne sur 2).
- Sélection par checkbox ; éviter le clic-pour-sélectionner qui empêche la copie.
- Densité : `Density::Compact` dès >20 lignes.

---

## 9. États vides & chargement

- **Loading** : `Spinner` pour < 1 s d'attente, `Skeleton` (forme de la donnée à venir) pour 1-5 s, `ProgressBar` quand on connaît la progression.
- **Empty** : `EmptyState` avec une icône évocatrice (`Icon::Leaf` pour "tout est calme", `Icon::Folder` pour "aucun fichier"…), un texte expliquant l'état + une action si applicable.
- **Error** : `Alert` au niveau `Error` avec une action "Retry" en `Button::secondary` ; ne jamais laisser seulement "Something went wrong".

---

## 10. Accessibilité : check-list rapide

- Contraste : déjà couvert par la palette (test `tests/contrast.rs`). N'introduisez pas de couleurs hors palette.
- Focus visible : tous les composants interactifs `egui_sauge` peignent un anneau `focus_ring` 2 px. Ne le supprimez pas.
- Touch target : 32 px (Comfortable) / 26 px (Compact). N'inventez pas plus petit pour les boutons primaires.
- Couleur jamais seule : préfixer chaque alerte/badge sémantique d'un icône (déjà fait par `Alert`, `Toast`, `Badge::leading(...)`).
- Texte : minimum 13 px pour les éléments cliquables. `Body` (14) par défaut.
- Animations : utilisateurs avec `reduce_motion` peuvent désactiver les transitions. (Hook futur sur `Spinner`, `Skeleton`.)

---

## 11. Patterns IT-spécifiques

### Health dashboard
- `Stat` pour les KPIs hauts (sessions, erreurs, p99, coût).
- `StatusDot::new(StatusLevel::Online).pulse()` pour chaque service.
- `LogLine` (avec timestamp) pour la queue d'événements récents.
- `Alert` (Warning/Error) au-dessus des KPIs si incident actif.

### Liste de ressources (servers, secrets, deployments…)
- Sidebar nav avec `NavItem`.
- `PageHeader` avec breadcrumb + action `Button::primary("Add server").leading(Icon::Plus)`.
- Filtres en haut, barre de recherche `InputField` + `Icon::Search`.
- Liste : `Card` par item OU table dense.
- Sélection d'un item → `SidePanel::right` avec détails (révocable) ; édition critique → `Dialog`.

### Workflow / pipeline
- Fil d'étapes : `Stat` ou cartes successives reliées par `Icon::ChevronRight`.
- En cours : `Spinner` ou `ProgressBar` à l'étape courante.
- Erreur : étape rouge + `Alert` Error avec action "Retry".

### Confirmation destructive
```rust
let mut delete_open = false;
if ui.add(Button::danger("Delete").leading(Icon::Trash)).clicked() {
    delete_open = true;
}
if delete_open {
    match ConfirmDialog::new("Supprimer le serveur api-eu-3 ?",
        "Tous les déploiements en cours seront interrompus.")
        .danger()
        .confirm_label("Supprimer")
        .show(ctx)
    {
        Some(true)  => { toasts.error("Serveur supprimé"); delete_open = false; }
        Some(false) => { delete_open = false; }
        None        => {}
    }
}
```

---

## 12. Anti-patterns à éviter

- ❌ 3 boutons primaires côte à côte → un seul, les autres en secondary/ghost.
- ❌ Modal qui contient un autre modal.
- ❌ Toast qui dure 30 s parce que le message est important → utiliser `Alert` à la place.
- ❌ Couleur sémantique sans icône (rouge ≠ "erreur" pour un daltonien).
- ❌ Table avec >5 colonnes sans densité Compact.
- ❌ Sidebar avec >12 items sans groupement → utiliser un `Section`-like header dans la nav.
- ❌ Inputs sans label visible (le placeholder ne remplace pas le label).
- ❌ "OK" comme libellé de bouton primaire — préférer le verbe d'action ("Save", "Delete", "Send").
- ❌ Mélanger les conventions de bouton (annuler tantôt à gauche, tantôt à droite).
- ❌ Utiliser `Icon::Custom` pour reproduire un icône Phosphor existant — utiliser `Icon::Glyph(phosphor::regular::X)`.

---

## 13. Internationalisation

`egui_sauge` ne porte **pas** de runtime i18n complet : la lib n'émet qu'une poignée de chaînes (libellés `StatusDot`, boutons par défaut de `ConfirmDialog`). Pour traduire SES propres chaînes, branchez n'importe quel crate i18n côté app (`fluent`, `rust-i18n`, `gettext-rs`…).

### Locale du DS

```rust
use egui_sauge::{Locale, set_locale};

set_locale(ctx, Locale::Fr);     // ou Locale::En (par défaut)
// peut être appelé à tout moment ; les composants concernés réagissent
// au prochain frame.
```

Locales fournies : `En` (par défaut), `Fr`.

### Ce qui est traduit

| Composant | Chaînes |
|---|---|
| `StatusDot` (sans `.label(...)`) | "Online" / "Degraded" / "Offline" / "Idle" |
| `ConfirmDialog` (sans labels custom) | bouton **Confirm** / **Cancel** |

Tout le reste — `Button` labels, `Alert` body, `Toasts`, `InputField` placeholders / helpers, titres de pages — est fourni par votre app. Donc votre stack i18n.

### Pattern recommandé pour les apps

```rust
use egui_sauge::{Locale, set_locale};
use my_app::i18n;     // votre crate i18n d'app (fluent, rust-i18n, …)

fn apply_user_locale(ctx: &egui::Context, lang: &str) {
    // 1. dire au DS quelle locale il doit utiliser pour SES strings
    set_locale(ctx, Locale::from_lang_code(lang));
    // 2. dire à VOTRE i18n quelle locale utiliser pour le reste
    i18n::set_active(lang);
}

// ailleurs, dans vos UIs :
ui.add(Button::primary(i18n::t("save_changes")));
ui.add(Alert::new(Level::Success, &i18n::t("deploy_done")));
```

### Détection automatique

```rust
// macOS / Linux : prendre la langue de l'OS si possible.
let lang = std::env::var("LANG").unwrap_or_else(|_| "en".into());
set_locale(ctx, Locale::from_lang_code(&lang));
```

`Locale::from_lang_code` accepte `"fr"`, `"fr-FR"`, `"french"` → `Fr` ; tout le reste → `En`.

### Ajouter une langue

Pour l'instant, seuls `En` et `Fr` sont bundlés. Ajouter une langue requiert d'éditer `src/theme/locale.rs` (~10 lignes par locale). Si vous en avez besoin, ouvrez une PR ou subclassez via vos propres `StatusDot::label(...)` et `ConfirmDialog::confirm_label(...)` côté app.

---

## 14. Pour aller plus loin

- API complète : `cargo doc --open` (tous les composants sont documentés).
- Showcase live : `cargo run --example showcase`.
- Décisions architecturales : `egui_sauge-spec.md`.
- Changelog : `CHANGELOG.md`.
