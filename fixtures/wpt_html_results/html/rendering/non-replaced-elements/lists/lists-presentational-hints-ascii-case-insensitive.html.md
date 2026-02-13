# html/rendering/non-replaced-elements/lists/lists-presentational-hints-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/lists-presentational-hints-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#lists:presentational-hints">
<link rel="help" href="https://drafts.csswg.org/selectors-4/#attribute-case">
<meta name="assert" content="ul@type + li@type values are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<ul type="circle"><li type="disc"></ul>
<ul type="circle"><li type="DiSc"></ul>
<ul type="circle"><li type="diſc"></ul>
<ul type="circle"><li type="square"></ul>
<ul type="circle"><li type="SqUaRe"></ul>
<ul type="circle"><li type="ſquare"></ul>
<script>
const li = document.querySelectorAll("li");

test(() => {
  assert_equals(getComputedStyle(li[0]).getPropertyValue("list-style-type"),
  	"disc", "lowercase valid");
  assert_equals(getComputedStyle(li[1]).getPropertyValue("list-style-type"),
  	"disc", "mixed case valid");
  assert_equals(getComputedStyle(li[2]).getPropertyValue("list-style-type"),
  	"circle", "non-ASCII invalid");
}, "keyword disc");

test(() => {
  assert_equals(getComputedStyle(li[3]).getPropertyValue("list-style-type"),
  	"square", "lowercase valid");
  assert_equals(getComputedStyle(li[4]).getPropertyValue("list-style-type"),
  	"square", "mixed case valid");
  assert_equals(getComputedStyle(li[5]).getPropertyValue("list-style-type"),
  	"circle", "non-ASCII invalid");
}, "keyword square");
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
  "source_name": "html/rendering/non-replaced-elements/lists/lists-presentational-hints-ascii-case-insensitive.html"
}
```
