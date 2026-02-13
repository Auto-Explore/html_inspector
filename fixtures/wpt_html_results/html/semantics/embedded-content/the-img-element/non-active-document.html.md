# html/semantics/embedded-content/the-img-element/non-active-document.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/non-active-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>img in non-active document should not perform loads</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>

<!-- Per load the image so that any loads in this test would be cached. -->
<img src=/images/green-1x1.png>

<!-- tests -->
<template>
<img>
</template>

<script>

onload = function() {
  async_test(function(t) {
    var p = new DOMParser();
    var d = p.parseFromString('<img>', 'text/html');
    var i = d.querySelector('img');
    i.onerror = t.unreached_func('got unexpected error event');
    i.onload = t.unreached_func('got unexpected load event');
    i.src = '/images/green-1x1.png';
    // delay to ensure there is no load/error event fired.
    t.step_timeout(t.step_func_done(), 0);
  }, "DOMParser");

  async_test(function(t) {
    var d = document.implementation.createHTMLDocument('');
    d.body.innerHTML = '<img>';
    var i = d.querySelector('img');
    i.onerror = this.unreached_func('got unexpected error event');
    i.onload = this.unreached_func('got unexpected load event');
    i.src = '/images/green-1x1.png';
    // delay to ensure there is no load/error event fired.
    t.step_timeout(t.step_func_done(), 0);
  }, "createHTMLDocument");

  async_test(function(t) {
    var template = document.querySelector('template');
    var i = template.content.querySelector('img');
    i.onerror = this.unreached_func('got unexpected error event');
    i.onload = this.unreached_func('got unexpected load event');
    i.src = '/images/green-1x1.png';
    // delay to ensure there is no load/error event fired.
    t.step_timeout(t.step_func_done(), 0);
  }, "<template>");
};

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 333,
        "byte_start": 302,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 366,
        "byte_start": 361,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 366,
        "byte_start": 361,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/embedded-content/the-img-element/non-active-document.html"
}
```
