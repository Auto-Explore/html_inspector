# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/video-default-object-width-constrained-by-max-height.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/video-default-object-width-constrained-by-max-height.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta name="assert" content="Video that has no intrinsic size should have width constrained by max-height">
<link rel="help" href="https://w3c.github.io/csswg-drafts/css-sizing-4/#aspect-ratio-size-transfers">
<link author="Sammy Gill" href="mailto:sammy.gill@apple.com">
<link rel="match" href="/css/reference/ref-filled-green-100px-square.xht">
<style>
  video {
    background-color: green;
    width: auto;
    height: auto;
    max-height: 100px;
  }
</style>
</head>
  <!-- aspect-ratio should be set to: auto 100 / 100-->
<body>
<p>Test passes if there is a filled green square and <strong>no red</strong>.</p>
<video width="100" height="100" src="/css/support/60x60-green.png">
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 301,
        "byte_start": 240,
        "col": 1,
        "line": 6
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/video-default-object-width-constrained-by-max-height.html"
}
```
