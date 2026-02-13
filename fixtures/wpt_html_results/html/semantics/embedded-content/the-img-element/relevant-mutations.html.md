# html/semantics/embedded-content/the-img-element/relevant-mutations.html

Counts:
- errors: 0
- warnings: 193
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/relevant-mutations.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img relevant mutations</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/relevant-mutations.js"></script>
<div id=log></div>

<!-- should invoke update the image data -->

<img data-desc="src set">
<img src="/images/green-2x2.png" data-desc="src changed">
<img src="/images/green-2x2.png" data-desc="src removed">

<img data-desc="srcset set">
<img srcset="/images/green-2x2.png" data-desc="srcset changed">
<img srcset="/images/green-2x2.png" data-desc="srcset removed">

<img data-desc="sizes set">
<img sizes="" data-desc="sizes changed">
<img sizes="" data-desc="sizes removed">

<img src="/images/green-2x2.png" data-desc="src set to same value">

<img data-desc="crossorigin absent to empty, src absent">
<img data-desc="crossorigin absent to anonymous, src absent">
<img data-desc="crossorigin absent to use-credentials, src absent">
<img crossorigin data-desc="crossorigin empty to absent, src absent">
<img crossorigin data-desc="crossorigin empty to use-credentials, src absent">
<img crossorigin=anonymous data-desc="crossorigin anonymous to absent, src absent">
<img crossorigin=anonymous data-desc="crossorigin anonymous to use-credentials, src absent">
<img crossorigin=use-credentials data-desc="crossorigin use-credentials to absent, src absent">
<img crossorigin=use-credentials data-desc="crossorigin use-credentials to empty, src absent">
<img crossorigin=use-credentials data-desc="crossorigin use-credentials to anonymous, src absent">
<img crossorigin=use-credentials data-desc="crossorigin use-credentials to invalid, src absent">

<img src="/images/green-2x2.png" data-desc="crossorigin absent to empty, src already set">
<img src="/images/green-2x2.png" data-desc="crossorigin absent to anonymous, src already set">
<img src="/images/green-2x2.png" data-desc="crossorigin absent to use-credentials, src already set">
<img src="/images/green-2x2.png" crossorigin data-desc="crossorigin empty to absent, src already set">
<img src="/images/green-2x2.png" crossorigin data-desc="crossorigin empty to use-credentials, src already set">
<img src="/images/green-2x2.png" crossorigin=anonymous data-desc="crossorigin anonymous to absent, src already set">
<img src="/images/green-2x2.png" crossorigin=anonymous data-desc="crossorigin anonymous to use-credentials, src already set">
<img src="/images/green-2x2.png" crossorigin=use-credentials data-desc="crossorigin use-credentials to absent, src already set">
<img src="/images/green-2x2.png" crossorigin=use-credentials data-desc="crossorigin use-credentials to empty, src already set">
<img src="/images/green-2x2.png" crossorigin=use-credentials data-desc="crossorigin use-credentials to anonymous, src already set">
<img src="/images/green-2x2.png" crossorigin=use-credentials data-desc="crossorigin use-credentials to invalid, src already set">

<img data-desc="referrerpolicy absent to no-referrer-when-downgrade, src absent">
<img data-desc="referrerpolicy absent to no-referrer, src absent">
<img referrerpolicy data-desc="referrerpolicy empty to no-referrer-when-downgrade, src absent">
<img referrerpolicy data-desc="referrerpolicy empty to no-referrer, src absent">
<img referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to absent, src absent">
<img referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to empty, src absent">
<img referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to no-referrer, src absent">
<img referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to invalid, src absent">
<img referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to absent, src absent">
<img referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to empty, src absent">
<img referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to no-referrer-when-downgrade, src absent">
<img referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to invalid, src absent">

<img src="/images/green-2x2.png" data-desc="referrerpolicy absent to no-referrer-when-downgrade, src already set">
<img src="/images/green-2x2.png" data-desc="referrerpolicy absent to no-referrer, src already set">
<img src="/images/green-2x2.png" referrerpolicy data-desc="referrerpolicy empty to no-referrer-when-downgrade, src already set">
<img src="/images/green-2x2.png" referrerpolicy data-desc="referrerpolicy empty to no-referrer, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to absent, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to empty, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to no-referrer, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer-when-downgrade data-desc="referrerpolicy no-referrer-when-downgrade to invalid, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to absent, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to empty, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to no-referrer-when-downgrade, src already set">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer data-desc="referrerpolicy no-referrer to invalid, src already set">

<img src="/images/green-2x2.png" data-desc="inserted into picture"><picture></picture>

<picture><img src="/images/green-2x2.png" data-desc="removed from picture"></picture>

<picture><img src="/images/green-2x2.png" data-desc="parent is picture, previous source inserted"></picture>

<picture><source><img src="/images/green-2x2.png" data-desc="parent is picture, previous source removed"></picture>

<picture><source><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has srcset set"></picture>
<picture><source srcset=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has srcset changed"></picture>
<picture><source srcset=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has srcset removed"></picture>

<picture><source><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has sizes set"></picture>
<picture><source sizes=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has sizes changed"></picture>
<picture><source sizes=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has sizes removed"></picture>

<picture><source><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has media set"></picture>
<picture><source media=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has media changed"></picture>
<picture><source media=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has media removed"></picture>

<picture><source><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has type set"></picture>
<picture><source type=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has type changed"></picture>
<picture><source type=""><img src="/images/green-2x2.png" data-desc="parent is picture, previous source has type removed"></picture>

<img srcset="/images/green-2x2.png" data-desc="srcset is set to same value">
<img srcset="/images/green-2x2.png" sizes data-desc="sizes is set to same value">

<img src="/images/green-2x2.png" data-desc="crossorigin state not changed: absent, removeAttribute">
<img src="/images/green-2x2.png" crossorigin data-desc="crossorigin state not changed: empty to anonymous">
<img src="/images/green-2x2.png" crossorigin=anonymous data-desc="crossorigin state not changed: anonymous to foobar">
<img src="/images/green-2x2.png" crossorigin=use-credentials data-desc="crossorigin state not changed: use-credentials to USE-CREDENTIALS">

<img src="/images/green-2x2.png" data-desc="referrerpolicy state not changed: absent, removeAttribute">
<img src="/images/green-2x2.png" referrerpolicy data-desc="referrerpolicy state not changed: empty to empty">
<img src="/images/green-2x2.png" referrerpolicy data-desc="referrerpolicy state not changed: empty to invalid">
<img src="/images/green-2x2.png" data-desc="referrerpolicy state not changed: absent to invalid">
<img src="/images/green-2x2.png" referrerpolicy=no-referrer data-desc="referrerpolicy state not changed: no-referrer to NO-REFERRER">
<img src="/images/green-2x2.png" referrerpolicy=foobar data-desc="referrerpolicy state not changed: invalid to other-invalid">

<img src="/images/green-2x2.png" data-desc="inserted into picture ancestor"><picture><span></span></picture>
<picture><span><img src="/images/green-2x2.png" data-desc="removed from picture ancestor"></span></picture>

<picture><span><img src="/images/green-2x2.png" data-desc="ancestor picture has a source inserted"></span></picture>
<picture><source><span><img src="/images/green-2x2.png" data-desc="ancestor picture has a source removed"></span></picture>

<picture><span><img src="/images/green-2x2.png" data-desc="ancestor picture; previous sibling source inserted"></span></picture>
<picture><span><source><img src="/images/green-2x2.png" data-desc="ancestor picture; previous sibling source removed"></span></picture>

<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following sibling source inserted"></picture>
<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following sibling source removed"><source></picture>

<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following sibling source has srcset set"><source></picture>

<img src="/images/green-2x2.png" data-desc="media on img set">
<img src="/images/green-2x2.png" data-desc="type on img set">
<img src="/images/green-2x2.png" data-desc="class on img set">
<img src="/images/green-2x2.png" data-desc="alt on img set">

<picture><source><img src="/images/green-2x2.png" data-desc="src on previous sibling source set"></picture>
<picture><source><img src="/images/green-2x2.png" data-desc="class on previous sibling source set"></picture>

<img src="/images/green-2x2.png" data-desc="inserted/removed children of img">

<picture><img src="/images/green-2x2.png" data-desc="picture is inserted; img has src"></picture><span></span>
<picture><img srcset="/images/green-2x2.png" data-desc="picture is inserted; img has srcset"></picture><span></span>
<picture><source srcset="/images/green-2x2.png"><img src="/images/green-2x2.png" data-desc="picture is inserted; img has previous sibling source"></picture><span></span>
<picture><img src="/images/green-2x2.png" data-desc="picture is inserted; img has following sibling source"><source srcset="/images/green-2x2.png"></picture><span></span>

<picture><img src="/images/green-2x2.png" data-desc="picture is removed; img has src"></picture>
<picture><img srcset="/images/green-2x2.png" data-desc="picture is removed; img has srcset"></picture>
<picture><source srcset="/images/green-2x2.png"><img src="/images/green-2x2.png" data-desc="picture is removed; img has previous sibling source"></picture>
<picture><img src="/images/green-2x2.png" data-desc="picture is removed; img has following sibling source"><source srcset="/images/green-2x2.png"></picture>

<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following img inserted"></picture>
<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following img removed"><img></picture>
<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following img has src set"><img></picture>
<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following img has srcset set"><img></picture>
<picture><img src="/images/green-2x2.png" data-desc="parent is picture, following img has sizes set"><img></picture>


<script>
onload = function() {

  t('src set', function(img) {
    img.src = '/images/green-2x2.png';
  }, 'load');

  t('src changed', function(img) {
    img.src = '/images/green-2x2.png ';
  }, 'load');

  t('src removed', function(img) {
    img.removeAttribute('src');
  }, 'timeout');

  t('srcset set', function(img) {
    img.srcset = '/images/green-2x2.png';
  }, 'load');

  t('srcset changed', function(img) {
    img.srcset = '/images/green-2x2.png ';
  }, 'load');

  t('srcset removed', function(img) {
    img.removeAttribute('srcset');
  }, 'timeout');

  t('sizes set', function(img) {
    img.sizes = '';
  }, 'timeout');

  t('sizes changed', function(img) {
    img.sizes = ' ';
  }, 'timeout');

  t('sizes removed', function(img) {
    img.removeAttribute('sizes');
  }, 'timeout');

  t('src set to same value', function(img) {
    img.src = '/images/green-2x2.png';
  }, 'load');

  // When src is absent, changing the crossorigin attribute state MUST NOT
  // generate events.

  t('crossorigin absent to empty, src absent', function(img) {
    img.crossOrigin = '';
  }, 'timeout');

  t('crossorigin absent to anonymous, src absent', function(img) {
    img.crossOrigin = 'anonymous';
  }, 'timeout');

  t('crossorigin absent to use-credentials, src absent', function(img) {
    img.crossOrigin = 'use-credentials';
  }, 'timeout');

  t('crossorigin empty to absent, src absent', function(img) {
    img.removeAttribute('crossorigin');
  }, 'timeout');

  t('crossorigin empty to use-credentials, src absent', function(img) {
    img.crossOrigin = 'use-credentials';
  }, 'timeout');

  t('crossorigin anonymous to absent, src absent', function(img) {
    img.removeAttribute('crossorigin');
  }, 'timeout');

  t('crossorigin anonymous to use-credentials, src absent', function(img) {
    img.crossOrigin = 'use-credentials';
  }, 'timeout');

  t('crossorigin use-credentials to absent, src absent', function(img) {
    img.removeAttribute('crossorigin');
  }, 'timeout');

  t('crossorigin use-credentials to empty, src absent', function(img) {
    img.crossOrigin = '';
  }, 'timeout');

  t('crossorigin use-credentials to anonymous, src absent', function(img) {
    img.crossOrigin = 'anonymous';
  }, 'timeout');

  t('crossorigin use-credentials to invalid, src absent', function(img) {
    img.crossOrigin = 'foobar';
  }, 'timeout');

  // When src is set, changing the crossorigin attribute state MUST generate
  // events.

  t('crossorigin absent to empty, src already set', function(img) {
    img.crossOrigin = '';
  }, 'load');

  t('crossorigin absent to anonymous, src already set', function(img) {
    img.crossOrigin = 'anonymous';
  }, 'load');

  t('crossorigin absent to use-credentials, src already set', function(img) {
    img.crossOrigin = 'use-credentials';
  }, 'load');

  t('crossorigin empty to absent, src already set', function(img) {
    img.removeAttribute('crossorigin');
  }, 'load');

  t('crossorigin empty to use-credentials, src already set', function(img) {
    img.crossOrigin = 'use-credentials';
  }, 'load');

  t('crossorigin anonymous to absent, src already set', function(img) {
    img.removeAttribute('crossorigin');
  }, 'load');

  t('crossorigin anonymous to use-credentials, src already set', function(img) {
    img.crossOrigin = 'use-credentials';
  }, 'load');

  t('crossorigin use-credentials to absent, src already set', function(img) {
    img.removeAttribute('crossorigin');
  }, 'load');

  t('crossorigin use-credentials to empty, src already set', function(img) {
    img.crossOrigin = '';
  }, 'load');

  t('crossorigin use-credentials to anonymous, src already set', function(img) {
    img.crossOrigin = 'anonymous';
  }, 'load');

  t('crossorigin use-credentials to invalid, src already set', function(img) {
    img.crossOrigin = 'foobar';
  }, 'load');

  // When src is absent, changing the referrerpolicy attribute state MUST NOT
  // generate events.

  t('referrerpolicy a
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 317,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 317,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 400,
        "byte_start": 343,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 458,
        "byte_start": 401,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 488,
        "byte_start": 460,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 488,
        "byte_start": 460,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 552,
        "byte_start": 489,
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
        "byte_end": 616,
        "byte_start": 553,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 645,
        "byte_start": 618,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 645,
        "byte_start": 618,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 686,
        "byte_start": 646,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 686,
        "byte_start": 646,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 686,
        "byte_start": 646,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 727,
        "byte_start": 687,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 727,
        "byte_start": 687,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 727,
        "byte_start": 687,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 796,
        "byte_start": 729,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 855,
        "byte_start": 798,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 855,
        "byte_start": 798,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 917,
        "byte_start": 856,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 917,
        "byte_start": 856,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 985,
        "byte_start": 918,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 985,
        "byte_start": 918,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1055,
        "byte_start": 986,
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
        "byte_end": 1055,
        "byte_start": 986,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1134,
        "byte_start": 1056,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1134,
        "byte_start": 1056,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1218,
        "byte_start": 1135,
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
        "byte_end": 1218,
        "byte_start": 1135,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1311,
        "byte_start": 1219,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1311,
        "byte_start": 1219,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1407,
        "byte_start": 1312,
        "col": 1,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1407,
        "byte_start": 1312,
        "col": 1,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1502,
        "byte_start": 1408,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1502,
        "byte_start": 1408,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1601,
        "byte_start": 1503,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1601,
        "byte_start": 1503,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1698,
        "byte_start": 1602,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1698,
        "byte_start": 1602,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1790,
        "byte_start": 1700,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1885,
        "byte_start": 1791,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1986,
        "byte_start": 1886,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2089,
        "byte_start": 1987,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2201,
        "byte_start": 2090,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2318,
        "byte_start": 2202,
        "col": 1,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2444,
        "byte_start": 2319,
        "col": 1,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2573,
        "byte_start": 2445,
        "col": 1,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2701,
        "byte_start": 2574,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2833,
        "byte_start": 2702,
        "col": 1,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2963,
        "byte_start": 2834,
        "col": 1,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3046,
        "byte_start": 2965,
        "col": 1,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3046,
        "byte_start": 2965,
        "col": 1,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3113,
        "byte_start": 3047,
        "col": 1,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3113,
        "byte_start": 3047,
        "col": 1,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3209,
        "byte_start": 3114,
        "col": 1,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3209,
        "byte_start": 3114,
        "col": 1,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3290,
        "byte_start": 3210,
        "col": 1,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3290,
        "byte_start": 3210,
        "col": 1,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3414,
        "byte_start": 3291,
        "col": 1,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3414,
        "byte_start": 3291,
        "col": 1,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3537,
        "byte_start": 3415,
        "col": 1,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3537,
        "byte_start": 3415,
        "col": 1,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3666,
        "byte_start": 3538,
        "col": 1,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3666,
        "byte_start": 3538,
        "col": 1,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3791,
        "byte_start": 3667,
        "col": 1,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3791,
        "byte_start": 3667,
        "col": 1,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3885,
        "byte_start": 3792,
        "col": 1,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3885,
        "byte_start": 3792,
        "col": 1,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3978,
        "byte_start": 3886,
        "col": 1,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3978,
        "byte_start": 3886,
        "col": 1,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4092,
        "byte_start": 3979,
        "col": 1,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4092,
        "byte_start": 3979,
        "col": 1,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4187,
        "byte_start": 4093,
        "col": 1,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4187,
        "byte_start": 4093,
        "col": 1,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4303,
        "byte_start": 4189,
        "col": 1,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4403,
        "byte_start": 4304,
        "col": 1,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4532,
        "byte_start": 4404,
        "col": 1,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4646,
        "byte_start": 4533,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4803,
        "byte_start": 4647,
        "col": 1,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4959,
        "byte_start": 4804,
        "col": 1,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5121,
        "byte_start": 4960,
        "col": 1,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5279,
        "byte_start": 5122,
        "col": 1,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5406,
        "byte_start": 5280,
        "col": 1,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5532,
        "byte_start": 5407,
        "col": 1,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5679,
        "byte_start": 5533,
        "col": 1,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5807,
        "byte_start": 5680,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5876,
        "byte_start": 5809,
        "col": 1,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5895,
        "byte_start": 5885,
        "col": 77,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5972,
        "byte_start": 5906,
        "col": 10,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6082,
        "byte_start": 5993,
        "col": 10,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6111,
        "byte_start": 6103,
        "col": 10,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6199,
        "byte_start": 6111,
        "col": 18,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6228,
        "byte_start": 6220,
        "col": 10,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6323,
        "byte_start": 6228,
        "col": 18,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6361,
        "byte_start": 6343,
        "col": 10,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6460,
        "byte_start": 6361,
        "col": 28,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6498,
        "byte_start": 6480,
        "col": 10,
        "line": 85
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6597,
        "byte_start": 6498,
        "col": 28,
        "line": 85
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6626,
        "byte_start": 6618,
        "col": 10,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6720,
        "byte_start": 6626,
        "col": 18,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6757,
        "byte_start": 6740,
        "col": 10,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6855,
        "byte_start": 6757,
        "col": 27,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6892,
        "byte_start": 6875,
        "col": 10,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6990,
        "byte_start": 6892,
        "col": 27,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7019,
        "byte_start": 7011,
        "col": 10,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7113,
        "byte_start": 7019,
        "col": 18,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7150,
        "byte_start": 7133,
        "col": 10,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7248,
        "byte_start": 7150,
        "col": 27,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7285,
        "byte_start": 7268,
        "col": 10,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7383,
        "byte_start": 7285,
        "col": 27,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7412,
        "byte_start": 7404,
        "col": 10,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7505,
        "byte_start": 7412,
        "col": 18,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7541,
        "byte_start": 7525,
        "col": 10,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7638,
        "byte_start": 7541,
        "col": 26,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7674,
        "byte_start": 7658,
        "col": 10,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7771,
        "byte_start": 7674,
        "col": 26,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7859,
        "byte_start": 7783,
        "col": 1,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7941,
        "byte_start": 7860,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “/images/green-2x2.png” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7941,
        "byte_start": 7860,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7941,
        "byte_start": 7860,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8043,
        "byte_start": 7943,
        "col": 1,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8151,
        "byte_start": 8044,
        "col": 1,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8270,
        "byte_start": 8152,
        "col": 1,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8410,
        "byte_start": 8271,
        "col": 1,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8515,
        "byte_start": 8412,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8625,
        "byte_start": 8516,
        "col": 1,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8737,
        "byte_start": 8626,
        "col": 1,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8835,
        "byte_start": 8738,
        "col": 1,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8969,
        "byte_start": 8836,
        "col": 1,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9096,
        "byte_start": 8970,
        "col": 1,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9174,
        "byte_start": 9098,
        "col": 1,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9189,
        "byte_start": 9183,
        "col": 86,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9206,
        "byte_start": 9196,
        "col": 99,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9222,
        "byte_start": 9216,
        "col": 10,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9297,
        "byte_start": 9222,
        "col": 16,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9314,
        "byte_start": 9304,
        "col": 98,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9331,
        "byte_start": 9325,
        "col": 10,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9415,
        "byte_start": 9331,
        "col": 16,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9432,
        "byte_start": 9422,
        "col": 107,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9450,
        "byte_start": 9442,
        "col": 10,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9456,
        "byte_start": 9450,
        "col": 18,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9539,
        "byte_start": 9456,
        "col": 24,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9556,
        "byte_start": 9546,
        "col": 114,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9573,
        "byte_start": 9567,
        "col": 10,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9669,
        "byte_start": 9573,
        "col": 16,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9686,
        "byte_start": 9676,
        "col": 119,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9702,
        "byte_start": 9696,
        "col": 10,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9805,
        "byte_start": 9710,
        "col": 24,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9822,
        "byte_start": 9812,
        "col": 126,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9931,
        "byte_start": 9833,
        "col": 10,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10048,
        "byte_start": 9951,
        "col": 10,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 10056,
        "byte_start": 10048,
        "col": 107,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10056,
        "byte_start": 10048,
        "col": 107,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10181,
        "byte_start": 10077,
        "col": 10,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 10189,
        "byte_start": 10181,
        "col": 114,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10189,
        "byte_start": 10181,
        "col": 114,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10263,
        "byte_start": 10201,
        "col": 1,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10325,
        "byte_start": 10264,
        "col": 1,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10388,
        "byte_start": 10326,
        "col": 1,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10449,
        "byte_start": 10389,
        "col": 1,
        "line": 131
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10468,
        "byte_start": 10460,
        "col": 10,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10548,
        "byte_start": 10468,
        "col": 18,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10576,
        "byte_start": 10568,
        "col": 10,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10658,
        "byte_start": 10576,
        "col": 18,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10748,
        "byte_start": 10670,
        "col": 1,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10837,
        "byte_start": 10759,
        "col": 10,
        "line": 138
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10954,
        "byte_start": 10870,
        "col": 10,
        "line": 139
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11124,
        "byte_start": 11026,
        "col": 49,
        "line": 140
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11256,
        "byte_start": 11157,
        "col": 10,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 11295,
        "byte_start": 11256,
        "col": 109,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11406,
        "byte_start": 11329,
        "col": 10,
        "line": 143
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11509,
        "byte_start": 11426,
        "col": 10,
        "line": 144
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11665,
        "byte_start": 11568,
        "col": 49,
        "line": 145
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11783,
        "byte_start": 11685,
        "col": 10,
        "line": 146
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 11822,
        "byte_start": 11783,
        "col": 108,
        "line": 146
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11930,
        "byte_start": 11843,
        "col": 10,
        "line": 148
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12036,
        "byte_start": 11950,
        "col": 10,
        "line": 149
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12041,
        "byte_start": 12036,
        "col": 96,
        "line": 149
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12041,
        "byte_start": 12036,
        "col": 96,
        "line": 149
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 12041,
        "byte_start": 12036,
        "col": 96,
        "line": 149
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12151,
        "byte_start": 12061,
        "col": 10,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12156,
        "byte_start": 12151,
        "col": 100,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12156,
        "byte_start": 12151,
        "col": 100,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 12156,
        "byte_start": 12151,
        "col": 100,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12269,
        "byte_start": 12176,
        "col": 10,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12274,
        "byte_start": 12269,
        "col": 103,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12274,
        "byte_start": 12269,
        "col": 103,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 12274,
        "byte_start": 12269,
        "col": 103,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12386,
        "byte_start": 12294,
        "col": 10,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12391,
        "byte_start": 12386,
        "col": 102,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12391,
        "byte_start": 12386,
        "col": 102,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 12391,
        "byte_start": 12386,
        "col": 102,
        "line": 152
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
  "source_name": "html/semantics/embedded-content/the-img-element/relevant-mutations.html"
}
```
