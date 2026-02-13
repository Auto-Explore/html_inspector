# html/semantics/popovers/popover-change-type.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-change-type.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9034">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/html/semantics/popovers/resources/popover-utils.js"></script>

<div id=mypopover>popover</div>

<script>
promise_test(async () => {
  const mypopover = document.getElementById('mypopover');

  mypopover.popover = "manual";
  mypopover.showPopover();

  await new Promise(resolve => {
    mypopover.addEventListener("beforetoggle", (e) => {
      if (e.newState === "closed") {
        mypopover.remove();
        requestAnimationFrame(() => {
          document.body.append(mypopover);
          mypopover.showPopover();
          resolve();
        });
      }
    }, {once: true});

    mypopover.popover = "auto";
  });

  assert_true(mypopover.matches(':popover-open'),
    'The popover should be open after the toggling sequence.');

  await sendEscape();
  assert_false(mypopover.matches(':popover-open'),
    'The popover should light dismiss because it is in the auto state.');
}, 'Changing the popover attribute should always update the auto/manual behavior.');
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
  "source_name": "html/semantics/popovers/popover-change-type.html"
}
```
