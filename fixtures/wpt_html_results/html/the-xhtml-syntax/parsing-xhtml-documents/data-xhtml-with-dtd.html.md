# html/the-xhtml-syntax/parsing-xhtml-documents/data-xhtml-with-dtd.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/the-xhtml-syntax/parsing-xhtml-documents/data-xhtml-with-dtd.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!--
    Any copyright is dedicated to the Public Domain.
    http://creativecommons.org/publicdomain/zero/1.0/
  -->
<html>
  <head>
    <title>
      Test that an XHTML document with a data: URL still handles the XHTML DTD
      properly even if the DTD URL is given as a relative URL.
    </title>
    <link rel="author" title="Boris Zbarsky" href="bzbarsky@mit.edu">
    <link rel="match" href="data-xhtml-with-dtd-ref.html">
  </head>
  <body>
    Test passes if it correctly shows &Aacute; in the subframe.
    <hr>
    <!-- Document in the subframe is:
<?xml version="1.0"?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    &Aacute;
  </body>
</html>
    -->
    <iframe src='data:application/xml,%3C%3Fxml%20version%3D%221.0%22%3F%3E%0A%3C!DOCTYPE%20html%20PUBLIC%20%22-%2F%2FW3C%2F%2FDTD%20XHTML%201.0%20Strict%2F%2FEN%22%20%22DTD%2Fxhtml1-strict.dtd%22%3E%0A%3Chtml%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxhtml%22%3E%0A%20%20%3Cbody%3E%0A%20%20%20%20%26Aacute%3B%0A%20%20%3C%2Fbody%3E%0A%3C%2Fhtml%3E%0A'></iframe>
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
  "source_name": "html/the-xhtml-syntax/parsing-xhtml-documents/data-xhtml-with-dtd.html"
}
```
