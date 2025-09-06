# CONTRIBUTING

This document is highly recommended to be read before contributing to `typed_ecs`.

## Commit Message Guidelines

To keep the commit history clean and aligned with issues, follow these rules:

### 1. Structure

A commit message consists of:

```text
<type>(<scope>): <short summary>
<detailed description>
```

### 2. Types

Use one of these **types**:

- **feat**: A new feature (matches a *Feature Request* issue)
- **fix**: A bug fix (matches a *Bug Report* issue)
- **chore**: Maintenance tasks (e.g., configs, templates, build scripts)
- **docs**: Documentation changes
- **refactor**: Code refactor without changing behavior
- **test**: Adding or improving tests

### 3. Scope

The **scope** indicates the feature or component affected:

- For features: use the **FeatureName** from the issue title.
- For bugs: use a short bug name or component name.

**Examples:**

```text
feat(pluginlist): implement PluginList trait
fix(add_plugin): resolve compile-error when adding a void plugin
chore(issue templates): add initial issue templates
```

### 4. Short Summary

- Start with a **verb** in lowercase (e.g., add, implement, fix, update).
- Keep it under **72 characters**.
- Do not end with a period.

### 5. Detailed Description

Explain:

- **What** you changed
- **Why** you changed it (if not obvious)
- **Impact** or important details

Use bullet points for clarity:

```text
This adds:
- PluginList trait with build_all
- add_plugin using type-level ZST composition
- Tests to ensure correct behavior
```

### 6. Link to Issues

Always link commits to issues using the following in the description:

```text
Closes #<issue_number>
```

Note: this line would close the issue only when your commit is merged in the `main` (default) branch.

### Examples

```text
chore(issue templates): add initial issue templates
Adds Github issue templates for:
- Feature Request
- Bug Report
- Question

This completes the first TODO of #2
```

```text
feat(pluginlist): implement PluginList trait with its helper traits and methods
This adds:
- PluginList trait with build_all, a method to build/allocate plugins for the ECS
- add_plugin, using type-level ZST composition (via PhantomData wrappers and tuple nesting)
- a few tests to ensure PluginList has the intended behavior

Closes #1
```
