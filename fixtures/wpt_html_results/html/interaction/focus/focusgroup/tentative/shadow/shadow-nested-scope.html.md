# html/interaction/focus/focusgroup/tentative/shadow/shadow-nested-scope.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/shadow/shadow-nested-scope.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>focusgroup: nested shadow focusgroup isolation</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div id=outer_fg focusgroup="toolbar inline">
  <button id=outer1 tabindex=0>Outer 1</button>
  <div id=shadow_host></div>
  <button id=outer2 tabindex=0>Outer 2</button>
</div>

<script>
  function deepActiveElement(root = document) {
    let a = root.activeElement;
    while (a && a.shadowRoot && a.shadowRoot.activeElement) {
      a = a.shadowRoot.activeElement;
    }
    return a;
  }
  promise_test(async t => {
    const host = document.getElementById('shadow_host');
    const sr = host.attachShadow({mode: 'open'});
    sr.innerHTML = `
      <div id=inner_fg focusgroup="menu inline">
        <button id=inner1 tabindex=0>Inner 1</button>
        <button id=inner2 tabindex=0>Inner 2</button>
      </div>`;
    const outer1 = document.getElementById('outer1');
    const outer2 = document.getElementById('outer2');
    const inner1 = sr.getElementById('inner1');
    const inner2 = sr.getElementById('inner2');

    await focusAndKeyPress(outer1, kArrowRight);
    assert_equals(document.activeElement, outer2, 'Outer navigation skips inner shadow scope');
    assert_equals(deepActiveElement(), outer2, 'Deep active element is outer2 after outer navigation');

    inner1.focus();
    await sendArrowKey(kArrowRight);
    assert_equals(sr.activeElement, inner2, 'Inner navigation advances');
    assert_equals(deepActiveElement(), inner2, 'Deep active element is inner2');
    await sendArrowKey(kArrowRight);
    assert_equals(sr.activeElement, inner2, 'No wrap inside inner scope');
    assert_equals(deepActiveElement(), inner2, 'Deep active element remains inner2');
  }, 'Nested shadow focusgroup is isolated from outer scope navigation');
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
  "source_name": "html/interaction/focus/focusgroup/tentative/shadow/shadow-nested-scope.html"
}
```
