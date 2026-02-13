# html/semantics/embedded-content/the-img-element/adoption.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/adoption.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Adopting an image updates the image data</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>

<!-- tests -->

<div id="adoptTest1"></div>
<picture id="adoptTest2">
<source srcset="/images/green-2x2.png">
</picture>

<script>
function resolve(url) {
  if (url === "") {
    return url;
  }
  var a = document.createElement('a');
  a.href = url;
  return a.href;
}

function t(desc, data, expect) {
  async_test(function(t) {
    var d = (new DOMParser()).parseFromString(data, 'text/html');
    var i = d.querySelector('img');
    i.onerror = this.unreached_func('got unexpected error event');
    i.onload = this.step_func_done(function() {
      assert_equals(i.currentSrc, resolve(expect));
    });
    var n = d.querySelector('[adopt-node]');
    document.adoptNode(n);
  }, desc);
}

onload = function() {

  t('img (src only)',
    '<img src="/images/green-1x1.png" adopt-node>',
    '/images/green-1x1.png');

  t('img (src only), parent is picture',
    '<picture adopt-node><img src="/images/green-1x1.png"></picture>',
    '/images/green-1x1.png');

  t('img (src only), previous sibling is source',
    '<picture adopt-node><source srcset="/images/green-1x1.png"><img src="/images/green-2x2.png"></picture>',
    '/images/green-1x1.png');

  t('img (srcset 1 cand)',
    '<img srcset="/images/green-1x1.png" adopt-node>',
    '/images/green-1x1.png');

  t('img (srcset 1 cand), parent is picture',
    '<picture adopt-node><img srcset="/images/green-1x1.png"></picture>',
    '/images/green-1x1.png');

  t('img (srcset 1 cand), previous sibling is source',
    '<picture adopt-node><source srcset="/images/green-1x1.png"><img srcset="/images/green-2x2.png"></picture>',
    '/images/green-1x1.png');

  async_test(function(t) {
    var d = (new DOMParser()).parseFromString('<template><img src="/images/green-1x1.png"></template>', 'text/html');
    var i = d.querySelector('template').content.querySelector('img').cloneNode(1);
    i.onerror = this.unreached_func('got unexpected error event');
    i.onload = this.step_func_done(function() {
      assert_equals(i.currentSrc, resolve('/images/green-1x1.png'));
    });

    document.getElementById('adoptTest1').appendChild(i);
  }, 'adopt a cloned img in template');

  async_test(function(t) {
    var preload = new Image();
    preload.src = '/images/green-1x1.png?' + Math.random();
    preload.onload = t.step_func(function() {
      var d = (new DOMParser()).parseFromString('<img src="' + preload.src + '">', 'text/html');
      var i = d.querySelector('img');
      i.onerror = this.unreached_func('got unexpected error event');
      i.onload = this.step_func_done(function() {
        assert_equals(i.currentSrc, resolve("/images/green-2x2.png"));
      });

      var p = document.getElementById('adoptTest2');
      p.appendChild(i);
    });
  }, 'adoption is from appendChild');
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 335,
        "byte_start": 325,
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
  "source_name": "html/semantics/embedded-content/the-img-element/adoption.html"
}
```
