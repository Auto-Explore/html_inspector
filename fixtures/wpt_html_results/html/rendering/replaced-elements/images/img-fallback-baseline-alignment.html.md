# html/rendering/replaced-elements/images/img-fallback-baseline-alignment.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/images/img-fallback-baseline-alignment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Image fallback baseline alignment</title>
<link rel="author" href="mailto:zhoupeng.1996@bytedance.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#images-3">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  div {
    background: red;
    line-height: 200px;
    width: 100px;
  }
  img {
    border-right: solid 30px black;
    width: 30px;
    height: 30px;
  }
</style>
<body>
  <div>
    <img src="">
  </div>
  <script>
    async_test(t => {
      const img = document.querySelector('img');
      const div = document.querySelector('div');
      img.src = '';
      img.onerror = t.step_func_done(() => {
        assert_greater_than(img.offsetTop, div.offsetTop);
        assert_less_than(img.offsetTop + img.offsetHeight, div.offsetTop + div.offsetHeight);
      });
    });
  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src.empty",
      "message": "Bad value “” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 550,
        "byte_start": 538,
        "col": 5,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 550,
        "byte_start": 538,
        "col": 5,
        "line": 22
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
  "source_name": "html/rendering/replaced-elements/images/img-fallback-baseline-alignment.html"
}
```
