# html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/pre-margin-block-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/pre-margin-block-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
html {
  writing-mode: vertical-rl;
}
blockquote, figure, listing, p, plaintext, pre, xmp {
  margin-block: 1em;
}
</style>

<pre>pre</pre>
<xmp>xmp</xmp>
<listing>listing</listing>
<blockquote>blockquote</blockquote>
<figure>figure</figure>
<p>p</p>
<plaintext>plaintext
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
  "source_name": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/pre-margin-block-ref.html"
}
```
