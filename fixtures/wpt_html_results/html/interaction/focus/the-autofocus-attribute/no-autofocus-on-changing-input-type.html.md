# html/interaction/focus/the-autofocus-attribute/no-autofocus-on-changing-input-type.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/no-autofocus-on-changing-input-type.html",
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

<input id="input1" autofocus>
<select><option>o1</option></select>

<script>
"use strict";

// WebKit had a bug that reattaching RenderObject triggered autofocus again.
// https://bugs.webkit.org/show_bug.cgi?id=68513
promise_test(async () => {
  const input1 = document.querySelector('input');
  const select = document.querySelector('select');

  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, input1);
  input1.onblur = () => { input1.type = 'password'; };
  select.focus();
  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, select);
}, 'Changing input type should not refocus on the element.');
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/no-autofocus-on-changing-input-type.html"
}
```
