# html/semantics/embedded-content/the-img-element/update-media.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-media.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>img update media</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    var t = async_test('set media after src updates selected image');

    var img;

    onload = t.step_func(function() {
        img = document.querySelector('img');
        img.addEventListener('load', t.step_func_done(onImgLoad));

        var source = document.querySelector('source[data-media]');
        source.setAttribute('media', source.getAttribute('data-media'));
    });

    function onImgLoad() {
        img.removeEventListener('load', onImgLoad);

        assert_true(img.currentSrc.indexOf(img.getAttribute('data-expect')) > -1);
    }

</script>

<div id="log"></div>
<picture>
    <source srcset="/images/fail.gif" data-media="(max-width: 1px)" />
    <source srcset="/images/smiley.png" />
    <img data-expect="/images/smiley.png">
</picture>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.picture.source.always_matching.disallowed",
      "message": "A “source” element that has a following sibling “source” element or “img” element with a “srcset” attribute must have a “media” attribute and/or “type” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 851,
        "byte_start": 785,
        "col": 5,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 937,
        "byte_start": 899,
        "col": 5,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 937,
        "byte_start": 899,
        "col": 5,
        "line": 31
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-media.html"
}
```
