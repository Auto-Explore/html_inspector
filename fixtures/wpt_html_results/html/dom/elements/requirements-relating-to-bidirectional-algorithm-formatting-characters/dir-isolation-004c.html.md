# html/dom/elements/requirements-relating-to-bidirectional-algorithm-formatting-characters/dir-isolation-004c.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/requirements-relating-to-bidirectional-algorithm-formatting-characters/dir-isolation-004c.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>The dir attribute: numbers isolated from preceding text, same direction</title>
<link rel='author' title='Richard Ishida' href='mailto:ishida@w3.org'>
<link rel="help" href='http://www.w3.org/TR/html5/dom.html#requirements-relating-to-the-bidirectional-algorithm'>
<link rel='match' href='reference/dir-isolation-004-ref.html'>
<meta name='assert' content='Numeric element content with a dir attribute is treated as a neutral character and directionally isolated from preceding text.'>
<style type="text/css">
.test, .ref { font-size: 150%; border: 1px solid orange; margin: 10px; margin-right: 200px; padding: 5px; clear: both; }
input { margin: 5px; }
</style>
</head>
<body>
<p class="instructions" dir="ltr">Test passes if the two boxes are identical.</p>
<!--Notes:
Key to entities used below:
&#x5d0; ... &#x5d5; - The first six Hebrew letters (strongly RTL).
&#x202d; - The LRO (left-to-right-override) formatting character.
&#x202c; - The PDF (pop directional formatting) formatting character; closes LRO.
The punctuation is moved around in the source to make it easier to do visual comparisons when the test is run.
-->
<div class="test">
<div dir="ltr">&#x5d0; <span dir="ltr">3</span></div>
<div dir="ltr">a <span dir="ltr">3</span></div>
<div dir="rtl">&#x5d0; <span dir="rtl">3</span></div>
<div dir="rtl">a <span dir="rtl">3</span></div>
</div>
<div class="ref">
<div dir="ltr">&#x202d;&#x5d0; 3&#x202c;</div>
<div dir="ltr">&#x202d;a 3&#x202c;</div>
<div dir="rtl">&#x202d;3 &#x5d0;&#x202c;</div>
<div dir="rtl">&#x202d;3 a&#x202c;</div>
</div>
</body></html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 579,
        "byte_start": 556,
        "col": 1,
        "line": 10
      }
    }
  ],
  "source_name": "html/dom/elements/requirements-relating-to-bidirectional-algorithm-formatting-characters/dir-isolation-004c.html"
}
```
