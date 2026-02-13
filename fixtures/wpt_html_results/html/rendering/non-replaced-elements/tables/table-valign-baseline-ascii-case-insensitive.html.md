# html/rendering/non-replaced-elements/tables/table-valign-baseline-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-valign-baseline-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#tables-2:presentational-hints">
<link rel="help" href="https://drafts.csswg.org/selectors-4/#attribute-case">
<meta name="assert" content="@valign values on table-related elements are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<table><tr><td valign="baseline">X</table>
<table><tr><td valign="BaSeLiNe">X</table>
<table><tr><td valign="baſeline">X</table>
<script>
const td = document.querySelectorAll("td");

test(() => {
  assert_equals(getComputedStyle(td[0]).getPropertyValue("vertical-align"),
  	"baseline", "lowercase valid");
  assert_equals(getComputedStyle(td[1]).getPropertyValue("vertical-align"),
  	"baseline", "mixed case valid");
  assert_equals(getComputedStyle(td[2]).getPropertyValue("vertical-align"),
  	"middle", "non-ASCII invalid");
}, "keyword baseline");
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-valign-baseline-ascii-case-insensitive.html"
}
```
