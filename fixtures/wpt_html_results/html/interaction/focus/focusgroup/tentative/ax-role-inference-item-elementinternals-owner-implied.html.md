# html/interaction/focus/focusgroup/tentative/ax-role-inference-item-elementinternals-owner-implied.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/ax-role-inference-item-elementinternals-owner-implied.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - ElementInternals radio item roles preserved (radiogroup mapping)</title>
<meta name="assert" content="ElementInternals supplied 'radio' roles must not be overridden by implied item mapping within a radiogroup focusgroup.">
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="rgOwner" focusgroup="radiogroup">
  <focusgroup-radio-internals id="radio1" tabindex="0"></focusgroup-radio-internals>
  <focusgroup-radio-internals id="radio2" tabindex="-1"></focusgroup-radio-internals>
</div>

<script>
class FocusgroupRadioInternals extends HTMLElement {
  constructor() {
    super();
    this.internals_ = this.attachInternals();
    this.internals_.role = 'radio';
    this.textContent = 'Choice';
  }
}
customElements.define('focusgroup-radio-internals', FocusgroupRadioInternals);
</script>

<script>
if (!window.accessibilityController) {
  test(() => { assert_true(true); }, 'accessibilityController not available (noop)');
} else {
  test(() => {
    const radio1AX = accessibilityController.accessibleElementById('radio1');
  // Blink exposes ARIA role="radio" as AXRadioButton in the accessibility tree.
  assert_equals(radio1AX.role, 'AXRole: AXRadioButton');
    const radio2AX = accessibilityController.accessibleElementById('radio2');
  assert_equals(radio2AX.role, 'AXRole: AXRadioButton');
  }, 'ElementInternals radio roles preserved (radiogroup case)');
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
  "source_name": "html/interaction/focus/focusgroup/tentative/ax-role-inference-item-elementinternals-owner-implied.html"
}
```
