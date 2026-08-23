# 03 – Shared components

← [Back to overview](README.md)

`src/components/` holds cross-page components that **are** allowed to know about the domain: they
can call API services, read contexts, and reference domain types. That is the single distinction
from [`design-system/`](02-design-system.md), which may not.

Decision rule when you're unsure where a component goes:

| Question | `design-system/` | `components/` |
|---|---|---|
| Does it import from `api/hooks/`? | never | yes |
| Does it read a `contexts/` value? | never | yes |
| Does it mention a domain type (`User`, `ApiKey`)? | never | yes |
| Is it used by more than one page? | yes | yes — otherwise it belongs to the page |
| Does it have a Storybook story? | mandatory | when it renders without heavy mocking |

A component used by exactly one page lives in that page's `components/` folder — see
[04 – Pages](04-pages-router.md). Move it up only on the second consumer.

---

## Directory tree

```
src/components/
├── errors/
│   ├── ErrorFallback/          # Root error-boundary UI
│   └── NotFound/
│
├── forms/                      # React Hook Form bindings over design-system inputs
│   ├── Form/
│   │   ├── Form.tsx            # <form> + FormProvider + zodResolver
│   │   └── index.ts
│   ├── FormField/              # label + control + error, wired to RHF
│   ├── FormDrawer/             # Drawer containing a form + submit footer
│   └── fields/
│       ├── TextField/
│       ├── SelectField/
│       └── SwitchField/
│
├── head/
│   └── Head/                   # Document title / meta
│
├── layout/                     # Chrome pieces used by src/layouts/
│   ├── AppHeader/
│   ├── AppSidebar/
│   └── UserMenu/               # domain-aware: reads the auth context
│
├── notifications/
│   ├── Notifications/          # Renders the Zustand notification store
│   └── Notification/
│
├── ConfirmationDialog/         # Dialog + destructive-action confirmation
├── DataTable/                  # Table + pagination + empty/loading states
│   ├── DataTable.tsx
│   ├── DataTableEmptyState.tsx
│   ├── DataTablePagination.tsx
│   ├── data-table.module.scss
│   ├── types.ts
│   └── index.ts
├── MarkdownPreview/
└── ProtectedRoute/             # Route guard reading the auth context
```

Folder structure inside a component is identical to the design system's: `Component.tsx`,
`Component.test.tsx`, optional `Component.stories.tsx`, `component.module.scss`, `index.ts`.

---

## Component with sub-parts

When a component grows past ~150 lines, split it into sibling files inside its own folder rather
than creating a parallel top-level component. The barrel decides what is public.

```
DataTable/
├── DataTable.tsx               # public
├── DataTableEmptyState.tsx     # internal
├── DataTablePagination.tsx     # internal
├── DataTable.test.tsx
├── data-table.module.scss
├── types.ts
└── index.ts                    # exports DataTable + its types only
```

If a sub-part itself needs a stylesheet and a test, promote it to a nested folder:

```
DataTable/
├── DataTable.tsx
├── index.ts
└── DataTablePagination/
    ├── DataTablePagination.tsx
    ├── DataTablePagination.test.tsx
    ├── data-table-pagination.module.scss
    └── index.ts
```

---

## A domain-aware component

The point of this layer: it fetches its own data. Handle loading and error states before the happy
path — a component that assumes `data` is defined is a runtime crash waiting for a slow network.

```tsx
// src/components/layout/UserMenu/UserMenu.tsx
import { Trans } from '@lingui/react/macro';

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser';
import { Dropdown, DropdownItem, DropdownTrigger } from '@/design-system/Dropdown';
import { Spinner } from '@/design-system/Spinner';
import { Avatar } from '@/design-system/Avatar';

import styles from './user-menu.module.scss';

export function UserMenu() {
    const { data: user, isLoading } = useCurrentUser();

    if (isLoading) {
        return <Spinner size="sm" />;
    }

    if (!user) {
        return null;
    }

    return (
        <Dropdown>
            <DropdownTrigger className={styles.trigger}>
                <Avatar name={`${user.firstName} ${user.lastName}`} />
            </DropdownTrigger>
            <DropdownItem onSelect={() => logout()}>
                <Trans>Log out</Trans>
            </DropdownItem>
        </Dropdown>
    );
}
```

Note the direction of the imports: `components/` → `design-system/` and `components/` →
`api/hooks/`. Never the reverse.

---

## Forms

Forms stay on **React Hook Form + Zod**. `components/forms/` binds them to the design-system
inputs so pages never touch RHF's low-level API.

```tsx
// src/components/forms/Form/Form.tsx
import { zodResolver } from '@hookform/resolvers/zod';
import { FormProvider, useForm, type UseFormReturn } from 'react-hook-form';
import type { z, ZodType } from 'zod';

type FormProps<TSchema extends ZodType> = {
    schema: TSchema;
    onSubmit: (values: z.infer<TSchema>) => void | Promise<void>;
    defaultValues?: Partial<z.infer<TSchema>>;
    children: (methods: UseFormReturn<z.infer<TSchema>>) => React.ReactNode;
};

export function Form<TSchema extends ZodType>({
    schema,
    onSubmit,
    defaultValues,
    children,
}: FormProps<TSchema>) {
    const methods = useForm<z.infer<TSchema>>({
        resolver: zodResolver(schema),
        defaultValues: defaultValues as z.infer<TSchema>,
    });

    return (
        <FormProvider {...methods}>
            <form onSubmit={methods.handleSubmit(onSubmit)} noValidate>
                {children(methods)}
            </form>
        </FormProvider>
    );
}
```

```tsx
// Usage in a page
const schema = z.object({
    email: z.email(),
    firstName: z.string().min(1),
});

<Form schema={schema} onSubmit={handleCreate}>
    {() => (
        <>
            <TextField name="email" label={t`Email`} />
            <TextField name="firstName" label={t`First name`} />
            <Button type="submit" disabled={isMutating}>
                <Trans>Create</Trans>
            </Button>
        </>
    )}
</Form>
```

The schema lives next to the form that uses it — in the page folder for a page-specific form, in
the component folder for a reusable one. Schemas describing an API payload live in
[`api/domains/<domain>/types.ts`](01-api.md) instead.

`FormField` is the piece that wires a control to RHF state and renders its error; every field
component builds on it, so error rendering and `aria-describedby` are handled once.

---

## Testing

Shared components are the layer where manual service mocks pay off — mock the service, assert the
rendering.

```tsx
// src/components/layout/UserMenu/UserMenu.test.tsx
import { screen } from '@testing-library/react';

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser';
import { render } from '@/test-utils/render';
import { buildUser } from '@/test-utils/fixtures/users';
import { UserMenu } from './UserMenu';

vi.mock('@/api/hooks/useApiCurrentUser');

it('renders the avatar for the signed-in user', () => {
    const user = buildUser({ firstName: 'Ada', lastName: 'Lovelace' });
    vi.mocked(useCurrentUser).mockReturnValue({
        data: user,
        error: undefined,
        isLoading: false,
        mutate: vi.fn(),
    });

    render(<UserMenu />);

    expect(
        screen.getByRole('button', { name: /ada lovelace/i }),
        `expected the trigger for ${user.firstName}, got: ${document.body.textContent}`,
    ).toBeVisible();
});

it('renders nothing when signed out', () => {
    vi.mocked(useCurrentUser).mockReturnValue({
        data: undefined,
        error: undefined,
        isLoading: false,
        mutate: vi.fn(),
    });

    const { container } = render(<UserMenu />);

    expect(container, 'signed-out menu should render nothing').toBeEmptyDOMElement();
});
```

Use `render` from `@/test-utils/render` (not Testing Library's directly) so i18n, SWR and router
providers are in place — see [06 – Tooling](06-tooling.md).
