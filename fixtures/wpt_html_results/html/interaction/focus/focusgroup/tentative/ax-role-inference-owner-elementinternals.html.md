# html/interaction/focus/focusgroup/tentative/ax-role-inference-owner-elementinternals.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/ax-role-inference-owner-elementinternals.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Owner implied AX role inference with ElementInternals author role</title>
<meta name="assert" content="A focusgroup owner whose role is set via ElementInternals should preserve that author supplied role and not receive an implied focusgroup role.">
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- The custom element will set an explicit author role (list) via ElementInternals. -->
<focusgroup-owner-internals id="fgInternals" focusgroup="toolbar">
</focusgroup-owner-internals>

<script>
class FocusgroupOwnerInternals extends HTMLElement {
  constructor() {
    super();
    this.internals_ = this.attachInternals();
    // Role list should be preserved instead of implied toolbar.
    this.internals_.role = 'list';
    this.innerHTML = '<div tabindex="0">Item</div>';
  }
}
customElements.define('focusgroup-owner-internals', FocusgroupOwnerInternals);
</script>

<script>
if (!window.accessibilityController) {
  test(() => { assert_true(true); }, 'accessibilityController not available (noop)');
} else {
  test(() => {
    const ax = accessibilityController.accessibleElementById('fgInternals');
    // Expect ElementInternals supplied list role, not implied toolbar role.
    assert_equals(ax.role, 'AXRole: AXList');
  }, 'ElementInternals author role is preserved over implied focusgroup role');
}
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/focusgroup/tentative/ax-role-inference-owner-elementinternals.html"
}
```
