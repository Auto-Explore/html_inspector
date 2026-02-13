# html/editing/the-hidden-attribute/hidden-1g.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-1g.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The hidden attribute</title>
<link rel=match href=hidden-1-ref.html>
<link rel=author title=Ms2ger href=mailto:Ms2ger@gmail.com>
<link rel=help href=https://html.spec.whatwg.org/multipage/#the-hidden-attribute>
<link rel=help href=https://html.spec.whatwg.org/multipage/#hidden-elements>
<p>This line should be visible.
<p hidden=hidden>This line should not be visible.
<p hidden=blue>This line should not be visible.
<p hidden=true>This line should not be visible.
<p hidden=false>This line should not be visible.
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/the-hidden-attribute/hidden-1g.html"
}
```
