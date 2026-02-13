# html/interaction/focus/the-autofocus-attribute/skip-not-fully-active.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/skip-not-fully-active.html",
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

<iframe srcdoc="<input autofocus><script>window.frameElement.remove();</script>"></iframe>

<script>
'use strict';

promise_test(async () => {
  let iframe = document.querySelector('iframe');
  let iframeDocument = iframe.contentDocument;
  await waitForLoad(window);
  assert_not_equals(document.activeElement, iframe);
  assert_equals(iframeDocument.activeElement, iframeDocument.body);
}, 'Autofocus element in not-fully-active document should be skipped while flusing.');
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/skip-not-fully-active.html"
}
```
