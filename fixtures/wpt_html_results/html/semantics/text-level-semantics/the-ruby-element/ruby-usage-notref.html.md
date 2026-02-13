# html/semantics/text-level-semantics/the-ruby-element/ruby-usage-notref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-ruby-element/ruby-usage-notref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Reference File</title>
<link rel="author" title="Intel" href="http://www.intel.com/">

<p>君くん子しは和わして同どうぜず</p>
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
  "source_name": "html/semantics/text-level-semantics/the-ruby-element/ruby-usage-notref.html"
}
```
