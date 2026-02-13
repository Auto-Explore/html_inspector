# html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute-values.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute-values.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The scrolling attribute</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-page">
<link rel="match" href="iframe-scrolling-attribute-values-ref.html">
<meta name="fuzzy" content="maxDifference=0-1;totalPixels=0-4">

<style>
    iframe {
        width: 100px;
        height: 100px;
    }
</style>

<p>This page tests the behavior of the <tt>scrolling</tt> attribute on
<tt>&lt;iframe&gt;</tt> elements which contain a page large enough to need to
be scrolled.</p>

<iframe src="support/big-page.html"></iframe>
<iframe src="support/big-page.html" scrolling></iframe>
<iframe src="support/big-page.html" scrolling=""></iframe>
<iframe src="support/big-page.html" scrolling="auto"></iframe>
<iframe src="support/big-page.html" scrolling="yes"></iframe>
<iframe src="support/big-page.html" scrolling="no"></iframe>
<iframe src="support/big-page.html" scrolling="noscroll"></iframe>
<iframe src="support/big-page.html" scrolling="off"></iframe>
<iframe src="support/big-page.html" scrolling="NoScRoLl"></iframe>
<iframe src="support/big-page.html" scrolling="bogus"></iframe>
<iframe src="support/big-page.html" scrolling="1234"></iframe>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.tt.obsolete",
      "message": "The “tt” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 456,
        "byte_start": 452,
        "col": 40,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.element.tt.obsolete",
      "message": "The “tt” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 488,
        "byte_start": 484,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute-values.html"
}
```
