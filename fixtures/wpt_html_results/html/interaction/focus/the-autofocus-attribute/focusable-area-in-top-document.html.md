# html/interaction/focus/the-autofocus-attribute/focusable-area-in-top-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/focusable-area-in-top-document.html",
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

<iframe srcdoc="<input><script>document.querySelector('input').focus();</script>"></iframe>

<script>
'use strict';

promise_test(async () => {
  await waitForLoad(window);
  let iframe = document.querySelector('iframe');
  assert_equals(document.activeElement, iframe, 'Prereq: IFRAME should be focused');

  let input = document.createElement('input');
  input.autofocus = true;
  document.body.appendChild(input);

  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, iframe, 'activeElement should not be changed');
  assert_not_equals(document.activeElement, input);
}, 'If topDocument\'s focused area is not topDocument, autofocus is not processed.');
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/focusable-area-in-top-document.html"
}
```
