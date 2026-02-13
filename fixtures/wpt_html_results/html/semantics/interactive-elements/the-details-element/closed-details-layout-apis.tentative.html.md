# html/semantics/interactive-elements/the-details-element/closed-details-layout-apis.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/closed-details-layout-apis.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/6466">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<details id=details>
  <div style="width:100px; height:100px; background-color:red" id=innerdiv></div>
</details>

<script>
test(() => {
  assert_not_equals(innerdiv.getBoundingClientRect().x, 0, 'x before open');
  assert_not_equals(innerdiv.getBoundingClientRect().y, 0, 'y before open');
  assert_not_equals(innerdiv.getBoundingClientRect().width, 0, 'width before open');
  assert_not_equals(innerdiv.getBoundingClientRect().height, 0, 'height before open');
  details.open = true;
  assert_not_equals(innerdiv.getBoundingClientRect().x, 0, 'x after open');
  assert_not_equals(innerdiv.getBoundingClientRect().y, 0, 'y after open');
  assert_not_equals(innerdiv.getBoundingClientRect().width, 0, 'width after open');
  assert_not_equals(innerdiv.getBoundingClientRect().height, 0, 'height after open');
  details.open = false;
  assert_not_equals(innerdiv.getBoundingClientRect().x, 0, 'x after close');
  assert_not_equals(innerdiv.getBoundingClientRect().y, 0, 'y after close');
  assert_not_equals(innerdiv.getBoundingClientRect().width, 0, 'width after close');
  assert_not_equals(innerdiv.getBoundingClientRect().height, 0, 'height after close');
}, `Verifies the layout results of elements inside a closed <details> based on the usage of content-visibility:hidden.`);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 352,
        "byte_start": 342,
        "col": 1,
        "line": 9
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
  "source_name": "html/semantics/interactive-elements/the-details-element/closed-details-layout-apis.tentative.html"
}
```
