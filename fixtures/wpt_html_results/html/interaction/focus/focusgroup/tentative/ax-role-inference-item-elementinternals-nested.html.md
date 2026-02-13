# html/interaction/focus/focusgroup/tentative/ax-role-inference-item-elementinternals-nested.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/ax-role-inference-item-elementinternals-nested.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Nested owners ElementInternals item roles preserved</title>
<meta name="assert" content="In nested focusgroups, ElementInternals supplied item roles (button/menuitem) remain preserved and are not coerced by owner inference.">
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="outerToolbar" focusgroup="toolbar">
  <focusgroup-toolbar-button-internals id="tbItem" tabindex="0"></focusgroup-toolbar-button-internals>
  <div id="innerMenu" focusgroup="menu">
    <focusgroup-menuitem-internals id="menuItem" tabindex="0"></focusgroup-menuitem-internals>
  </div>
</div>

<script>
class FocusgroupToolbarButtonInternals extends HTMLElement {
  constructor() {
    super();
    this.internals_ = this.attachInternals();
    this.internals_.role = 'button';
    this.textContent = 'Tool';
  }
}
customElements.define('focusgroup-toolbar-button-internals', FocusgroupToolbarButtonInternals);

class FocusgroupMenuItemInternals extends HTMLElement {
  constructor() {
    super();
    this.internals_ = this.attachInternals();
    this.internals_.role = 'menuitem';
    this.textContent = 'Action';
  }
}
customElements.define('focusgroup-menuitem-internals', FocusgroupMenuItemInternals);
</script>

<script>
if (!window.accessibilityController) {
  test(() => { assert_true(true); }, 'accessibilityController not available (noop)');
} else {
  test(() => {
    const tbAX = accessibilityController.accessibleElementById('tbItem');
    assert_equals(tbAX.role, 'AXRole: AXButton');
    const menuItemAX = accessibilityController.accessibleElementById('menuItem');
    assert_equals(menuItemAX.role, 'AXRole: AXMenuItem');
  }, 'Nested focusgroup ElementInternals item roles preserved');
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
  "source_name": "html/interaction/focus/focusgroup/tentative/ax-role-inference-item-elementinternals-nested.html"
}
```
