# Adding an icon

Read this only when adding a new icon, not for a general-purpose primitive.

SVGs live in `Icon/resources/` and become components through `makeIcon`, which sets a default size,
marks the SVG decorative and forwards the rest of the native SVG props:

```tsx
import TrashSvg from './resources/trash.svg?react'
export const TrashIcon = makeIcon(TrashSvg, 'TrashIcon')
```

See [src/design-system/Icon/makeIcon.tsx](../../../frontend/src/design-system/Icon/makeIcon.tsx).

Icons are `aria-hidden` by default — an icon next to a label is decorative. An icon-only control
carries its own `aria-label` on the **button** (`IconButton` requires it), never on the SVG.
