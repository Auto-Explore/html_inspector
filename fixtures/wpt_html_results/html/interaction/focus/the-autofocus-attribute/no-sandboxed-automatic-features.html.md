# html/interaction/focus/the-autofocus-attribute/no-sandboxed-automatic-features.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/no-sandboxed-automatic-features.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/utils.js"></script>

<iframe sandbox srcdoc="<input autofocus>"></iframe>

<script>
'use strict';

promise_test(async () => {
  await waitForLoad(window);
  await waitUntilStableAutofocusState();
  assert_not_equals(document.activeElement, document.querySelector('iframe'));
}, 'If the sandboxed automatic features browsing context flag is set, ' +
    'autofocus in the browsing context should not be handled.');
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/no-sandboxed-automatic-features.html"
}
```
