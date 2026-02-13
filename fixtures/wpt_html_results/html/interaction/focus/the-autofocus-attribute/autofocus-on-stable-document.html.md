# html/interaction/focus/the-autofocus-attribute/autofocus-on-stable-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/autofocus-on-stable-document.html",
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

<body>
<script>
'use strict';

promise_test(async t => {
  await waitForLoad(window);
  await timeOut(t, 1000);
  let element = document.createElement('input');
  element.autofocus = true;
  document.body.appendChild(element);
  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, element);
}, 'Autofocus should work if an element with autofocus is inserted into a ' +
    'document which was loaded some time ago.');
</script>
</body>
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/autofocus-on-stable-document.html"
}
```
