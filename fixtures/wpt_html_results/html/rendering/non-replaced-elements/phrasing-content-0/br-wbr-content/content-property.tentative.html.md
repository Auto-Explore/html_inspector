# html/rendering/non-replaced-elements/phrasing-content-0/br-wbr-content/content-property.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/phrasing-content-0/br-wbr-content/content-property.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href=/fonts/ahem.css>
<link rel=match href=/css/reference/pass_if_square_96px_black.html>
<link rel=help href=https://github.com/whatwg/html/issues/2291>
<link rel=help href=https://drafts.csswg.org/css-content/#content-property>
<style>
.test, .ref {
  font: 16px/1 Ahem;
  margin: 0;
}
.test br {
  /* This should have no affect. Per css-content, <string> when applied to elements. */
  content: "";
}
</style>
<p>Test passes if there is a square below.</p>
<p class=test>xxxxxx<br>xxxxxx<br>xxxxxx</p>
<p class=ref>xxxxxx<br>xxxxxx<br>xxxxxx</p>
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
  "source_name": "html/rendering/non-replaced-elements/phrasing-content-0/br-wbr-content/content-property.tentative.html"
}
```
