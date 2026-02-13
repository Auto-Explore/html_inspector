# html/semantics/permission-element/bounded-sizes-reftest-ref.html

Counts:
- errors: 0
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/bounded-sizes-reftest-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta charset=utf-8>
<body>
  <div>
    The permission element should have some limits for the min/max-width/height:
    <ul>
    <li>min-width should be sufficient to fit the element text (depends on user agent implementation)</li>
    <li>max-width should be at most 3x min-width</li>
    <li>min-height should be sufficient to fit the element text (1em)</li>
    <li>max-height should be at most 3x min-height</li>
    <li>padding-left/top only work with width/height: auto and are at most 5em/1em</li>
    <li>padding-right/bottom are copied over from padding-left/top in this case</li>
    </ul>
  </div>

  <style>
    #id1 {
      font-size: 10px;
      height: 10px;
      border: 0px;
      /* width set via JS */
    }
    #id2 {
      font-size: 10px;
      height: 30px;
      border: 0px;
      /* width set via JS */
    }
    #id3 {
      font-size: 10px;
      color:black;
      background-color: black;
      border: 1000px solid darkmagenta;

      /* Used to compute width which will then have the padding
         artificially added in JS */
      width: fit-content;
      height: fit-content;
    }
  </style>

  <div><permission id="id1" type="geolocation"></permission></div>
  <div><permission id="id2" type="camera"></permission></div>
  <div><permission id="id3" type="microphone"></permission></div>

<script>
  let el = document.getElementById("id1");
  el.style.width = getComputedStyle(el).minWidth;

  el = document.getElementById("id2");
  el.style.width = getComputedStyle(el).maxWidth;

  el = document.getElementById("id3");
  let w = getComputedStyle(el).width;
  let h = getComputedStyle(el).height;
  el.style.width = `calc(${w} + 100px)`; // 100px is 2 * 5em;
  el.style.height = `calc(${h} + 10px)`; // 10px is 2 * 5px; (the padding set in the test)
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 643,
        "byte_start": 636,
        "col": 3,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1204,
        "byte_start": 1164,
        "col": 8,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1204,
        "byte_start": 1164,
        "col": 8,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1266,
        "byte_start": 1231,
        "col": 8,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1266,
        "byte_start": 1231,
        "col": 8,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1332,
        "byte_start": 1293,
        "col": 8,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1332,
        "byte_start": 1293,
        "col": 8,
        "line": 45
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
  "source_name": "html/semantics/permission-element/bounded-sizes-reftest-ref.html"
}
```
