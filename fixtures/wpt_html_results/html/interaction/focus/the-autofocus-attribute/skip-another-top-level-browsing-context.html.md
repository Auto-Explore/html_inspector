# html/interaction/focus/the-autofocus-attribute/skip-another-top-level-browsing-context.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/skip-another-top-level-browsing-context.html",
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
<script>
'use strict';

promise_test(async () => {
  let w = window.open('resources/moving-autofocus-to-parent.html');
  await waitForLoad(w);
  await waitUntilStableAutofocusState(w);
  assert_equals(w.document.activeElement, w.document.body);
  assert_equals(document.activeElement, document.body);
  w.close();
}, 'Autofocus elements queued in another top-level browsing context\'s ' +
   'documents should be skipped.');
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/skip-another-top-level-browsing-context.html"
}
```
