# html/interaction/focus/the-autofocus-attribute/spin-by-blocking-style-sheet.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/spin-by-blocking-style-sheet.html",
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

<input id="first" autofocus>
<input id="second" autofocus>

<link rel="stylesheet" href="resources/erase-first.css?pipe=trickle(d1)">

<script>
'use strict';

promise_test(async () => {
  await waitForLoad(window);
  await waitForAnimationFrame();
  assert_equals(document.activeElement.id, 'second');
}, 'Script-blocking style sheet should pause flushing autofocus candidates.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 224,
        "byte_start": 195,
        "col": 1,
        "line": 7
      }
    },
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/spin-by-blocking-style-sheet.html"
}
```
