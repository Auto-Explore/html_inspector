# html/semantics/text-level-semantics/the-ruby-element/ruby-usage.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-ruby-element/ruby-usage.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML test: ruby - mark phrasing content</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="mismatch" href="ruby-usage-notref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-ruby-element"/>

<p><ruby>君<rt>くん</ruby><ruby>子<rt>し</ruby>は<ruby>和<rt>わ</ruby>して<ruby>同<rt>どう</ruby>ぜず</p>
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
  "source_name": "html/semantics/text-level-semantics/the-ruby-element/ruby-usage.html"
}
```
