# html/semantics/embedded-content/the-img-element/relevant-mutations-lazy.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/relevant-mutations-lazy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img relevant mutations, lazy-loaded</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/relevant-mutations.js"></script>
<div id=log></div>

<img src="/images/green-2x2.png"> <!-- block the window load event -->

<!-- should invoke update the image data, but omit events for layout changes -->
<!-- but also see https://github.com/whatwg/html/issues/8492 -->

<img srcset="/images/green-2x2.png 2w" sizes="auto" width="2" loading="lazy" data-desc="width attribute changes">

<img srcset="/images/green-2x2.png 2w" sizes="auto" style="width: 2px" loading="lazy" data-desc="width property changes">

<div style="width: 2px">
  <img srcset="/images/green-2x2.png 2w" sizes="auto" style="width: 100%" loading="lazy" data-desc="percentage width, CB width changes">
</div>

<img srcset="/images/green-2x2.png 2w" sizes="auto" style="height: 2px; aspect-ratio: 2 / 2" loading="lazy" data-desc="height property changes (with aspect-ratio)">

<img srcset="/images/green-2x2.png 2w" sizes="auto" style="width: 2px" loading="lazy" data-desc="being rendered changes">

<!-- should not invoke update the image data -->

<img srcset="/images/green-2x2.png 2w" sizes="auto" style="width: 2px" loading="lazy" data-desc="loading attribute state changes">

<img srcset="/images/green-2x2.png 2w" sizes="auto" style="width: 2px" loading="lazy" data-desc="display property changes to inline-block">

<img srcset="/images/green-2x2.png 2w" sizes="auto" style="width: 2px" loading="lazy" data-desc="loading attribute changes to LAZY">

<script>
const rAF = () => new Promise(resolve => requestAnimationFrame(resolve));

onload = async function() {

  await rAF();
  await rAF();

  // lazy-loaded images should have fired their first 'load' event at this point.

  t('width attribute changes', function(img) {
    img.width = '4';
  }, 'load');

  t('width property changes', function(img) {
    img.style.width = '4px';
  }, 'timeout');

  t('percentage width, CB width changes', function(img) {
    img.parentNode.style.width = '4px';
  }, 'timeout');

  t('height property changes (with aspect-ratio)', function(img) {
    img.style.height = '4px';
  }, 'timeout');

  t('loading attribute state changes', function(img) {
    img.loading = 'eager';
  }, 'timeout');

  t('being rendered changes', function(img) {
    img.style.display = 'none';
  }, 'timeout');

  t('display property changes to inline-block', function(img) {
    img.style.display = 'inline-block';
  }, 'timeout');

  t('loading attribute changes to LAZY', function(img) {
    img.setAttribute('loading', 'LAZY');
  }, 'timeout');

  done();
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
        "byte_end": 317,
        "byte_start": 284,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 616,
        "byte_start": 503,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 739,
        "byte_start": 618,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 902,
        "byte_start": 768,
        "col": 3,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1075,
        "byte_start": 911,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1198,
        "byte_start": 1077,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1380,
        "byte_start": 1250,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1521,
        "byte_start": 1382,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1655,
        "byte_start": 1523,
        "col": 1,
        "line": 32
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
  "source_name": "html/semantics/embedded-content/the-img-element/relevant-mutations-lazy.html"
}
```
