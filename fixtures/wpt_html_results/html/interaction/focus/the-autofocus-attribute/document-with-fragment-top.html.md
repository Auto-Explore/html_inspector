# html/interaction/focus/the-autofocus-attribute/document-with-fragment-top.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/document-with-fragment-top.html",
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

<iframe src="resources/frame-with-autofocus-element.html#top"></iframe>

<script>
'use strict';

promise_test(async () => {
  await waitForLoad(window);
  const iframe = document.querySelector('iframe');
  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, iframe,
      'Autofocus elements in iframes should be focused.');
  const doc = iframe.contentDocument;
  assert_true(!doc.querySelector(':target'));

  let input = document.createElement('input');
  input.autofocus = true;
  document.body.appendChild(input);
  await waitUntilStableAutofocusState();
  assert_not_equals(document.activeElement, input);
}, 'Autofocus elements in iframed documents with "top" fragments should work.');

promise_test(async () => {
  let w = window.open('resources/frame-with-autofocus-element.html#top');
  await waitForLoad(w);
  await waitUntilStableAutofocusState(w);
  assert_not_equals(w.document.activeElement, w.document.body);
  w.close();
}, 'Autofocus elements in top-level browsing context\'s documents with "top" fragments should work.');
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/document-with-fragment-top.html"
}
```
