# html/rendering/non-replaced-elements/the-hr-element-0/size-with-color-or-noshade.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-hr-element-0/size-with-color-or-noshade.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-hr-element-2" />
<title>hr elements: Tests behaviour of a size attribute with color/noshade attributes present</title>
<link rel="author" title="Simon Wülker" href="mailto:simon.wuelker@arcor.de">
<link rel="match" href="/html/rendering/non-replaced-elements/the-hr-element-0/size-with-color-or-noshade-ref.html">
<meta name="assert" content="This checks that the size attribute of a hr element changes the border widths when color/noshade attributes are present">
<body>
    <hr size=50 color="black">
    <hr size=50 color="totally-not-a-color">
    <hr size=50 noshade>
    <hr size=50 color="black" noshade>
</body>
</html>

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
  "source_name": "html/rendering/non-replaced-elements/the-hr-element-0/size-with-color-or-noshade.html"
}
```
