# html/rendering/non-replaced-elements/the-frameset-and-frame-elements/exceed-then-not-exceed.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/exceed-then-not-exceed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
promise_test(async () => {
  await new Promise(resolve => document.addEventListener('DOMContentLoaded', resolve, {once:true}));
  await new Promise(resolve => requestAnimationFrame(resolve));
  await new Promise(resolve => requestAnimationFrame(resolve));
  // #fs1, #container, and #child were laid out.

  // Move #child.
  // It makes #container dirty, and it exceeds from #fs1's 1x1 grid.
  document.querySelector('#fs1').insertBefore(
      document.querySelector('#child'), document.querySelector('#container'));
  await new Promise(resolve => requestAnimationFrame(resolve));
  await new Promise(resolve => requestAnimationFrame(resolve));

  // Removing #child makes #container visible again.
  document.querySelector('#child').remove();
  await new Promise(resolve => requestAnimationFrame(resolve));
  await new Promise(resolve => requestAnimationFrame(resolve));
}, 'No crash when a dirty FRAMESET exceeds from the grid then fits in it again');
</script>
</head>
<frameset id="fs1">
<frameset id="container">
<frameset id="child"></frameset>
</frameset>
</frameset>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 1138,
        "byte_start": 1119,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 1164,
        "byte_start": 1139,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 1186,
        "byte_start": 1165,
        "col": 1,
        "line": 29
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
  "source_name": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/exceed-then-not-exceed.html"
}
```
