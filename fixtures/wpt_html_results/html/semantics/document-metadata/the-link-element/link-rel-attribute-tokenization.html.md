# html/semantics/document-metadata/the-link-element/link-rel-attribute-tokenization.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-rel-attribute-tokenization.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>

<link id=light-link rel=stylesheet href=resources/link-rel-attribute.css>
<div id=light-div class=green></div>

<script>

function testLinkRelModification(testDiv, testLink) {
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 128, 0)");
  testLink.setAttribute("rel", "test\u000Bstylesheet");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 0, 0)");
  testLink.setAttribute("rel", "test\u000Cstylesheet");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 128, 0)");
  testLink.removeAttribute("rel");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 0, 0)");
  testLink.setAttribute("rel", "\u0009\u000A\u000C\u000D\u0020stylesheet");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 128, 0)");
  testLink.removeAttribute("rel");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 0, 0)");
  testLink.setAttribute("rel", "\u0009\u000A\u000C\u000D\u0020stylesheet\u0009\u000A\u000C\u000D\u0020††††");
  assert_equals(getComputedStyle(testDiv).color, "rgb(0, 128, 0)");
}

test (() => {
  testLinkRelModification(document.querySelector("#light-div"),
                          document.querySelector("#light-link"));
}, "The rel attribute needs to handle ASCII whitespace correctly");

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
  "source_name": "html/semantics/document-metadata/the-link-element/link-rel-attribute-tokenization.html"
}
```
