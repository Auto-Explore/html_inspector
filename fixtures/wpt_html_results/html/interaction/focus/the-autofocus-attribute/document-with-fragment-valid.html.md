# html/interaction/focus/the-autofocus-attribute/document-with-fragment-valid.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/document-with-fragment-valid.html",
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

<iframe src="resources/frame-with-anchor.html"></iframe>

<script>
'use strict';

promise_test(async () => {
  await waitForLoad(window);
  const iframe = document.querySelector('iframe');
  iframe.contentWindow.location.hash = 'anchor1';
  await waitForEvent(iframe.contentWindow, 'hashchange');
  const doc = iframe.contentDocument;
  assert_true(!!doc.querySelector(':target'));

  let input = doc.createElement('input');
  input.autofocus = true;
  doc.body.appendChild(input);
  await waitUntilStableAutofocusState();
  assert_not_equals(doc.activeElement, input);
  iframe.remove();
}, 'Autofocus elements in iframed documents with URL fragments should be skipped. (id matches)');

promise_test(async () => {
  let iframe = await waitForIframeLoad("resources/frame-with-a.html");
  iframe.contentWindow.location.hash = 'anchor1';
  await waitForEvent(iframe.contentWindow, 'hashchange');
  const doc = iframe.contentDocument;
  assert_true(!!doc.querySelector(':target'));

  let input = doc.createElement('input');
  input.autofocus = true;
  doc.body.appendChild(input);
  await waitUntilStableAutofocusState();
  assert_not_equals(doc.activeElement, input);
  iframe.remove();
}, 'Autofocus elements in iframed documents with URL fragments should be skipped.(a element)');

promise_test(async () => {
  let w = window.open('resources/frame-with-anchor.html');
  await waitForLoad(w);
  w.location.hash = 'anchor1';
  await waitForEvent(w, 'hashchange');
  const doc = w.document;
  assert_true(!!doc.querySelector(':target'));

  let input = doc.createElement('input');
  input.autofocus = true;
  doc.body.appendChild(input);
  await waitUntilStableAutofocusState();
  assert_not_equals(doc.activeElement, input);
  w.close();
}, 'Autofocus elements in top-level browsing context\'s documents with URL fragments should be skipped.');
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/document-with-fragment-valid.html"
}
```
