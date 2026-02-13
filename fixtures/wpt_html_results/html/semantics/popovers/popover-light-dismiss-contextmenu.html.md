# html/semantics/popovers/popover-light-dismiss-contextmenu.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-light-dismiss-contextmenu.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/10905">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<div id=target>target</div>
<div id=popover popover=auto>popover</div>

<script>
const target = document.getElementById('target');
const popover = document.getElementById('popover');

promise_test(async () => {
  let contextmenuFired = false;
  target.addEventListener('contextmenu', event => {
    event.preventDefault();
    popover.showPopover();
    contextmenuFired = true;
  });

  const actions = new test_driver.Actions();
  await actions.pointerMove(0, 0, {origin: target})
    .pointerDown({button: actions.ButtonType.RIGHT})
    .pointerUp({button: actions.ButtonType.RIGHT})
    .send();

  assert_true(contextmenuFired, 'right clicking should fire a contextmenu event.');
  assert_true(popover.matches(':popover-open'), 'popover should be open.');
}, 'Popovers should not be light dismissed during contextmenu event.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/popover-light-dismiss-contextmenu.html"
}
```
