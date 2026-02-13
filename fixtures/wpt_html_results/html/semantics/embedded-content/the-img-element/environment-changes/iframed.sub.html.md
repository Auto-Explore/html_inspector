# html/semantics/embedded-content/the-img-element/environment-changes/iframed.sub.html

Counts:
- errors: 0
- warnings: 22
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/environment-changes/iframed.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<img
data-desc="img (no src)"
data-narrow=""
data-wide=""
data-no-change>

<img src=""
data-desc="img (empty src)"
data-narrow=""
data-wide=""
data-no-change>

<img src="/images/broken.png?30-{{GET[id]}}"
data-desc="img (src only) broken image"
data-narrow="/images/broken.png?30-{{GET[id]}}"
data-wide="/images/broken.png?30-{{GET[id]}}"
data-no-change>

<img src="/images/green-1x1.png?40-{{GET[id]}}"
data-desc="img (src only) valid image"
data-narrow="/images/green-1x1.png?40-{{GET[id]}}"
data-wide="/images/green-1x1.png?40-{{GET[id]}}"
data-no-change>

<img srcset="/images/broken.png?50-{{GET[id]}}"
data-desc="img (srcset 1 cand) broken image"
data-narrow="/images/broken.png?50-{{GET[id]}}"
data-wide="/images/broken.png?50-{{GET[id]}}"
data-no-change>

<img srcset="/images/green-1x1.png?60-{{GET[id]}}"
data-desc="img (srcset 1 cand) valid image"
data-narrow="/images/green-1x1.png?60-{{GET[id]}}"
data-wide="/images/green-1x1.png?60-{{GET[id]}}"
data-no-change>

<picture>
<source media="(max-width:500px)" srcset="/images/broken.png?70-{{GET[id]}}">
<img src="/images/broken.png?71-{{GET[id]}}"
data-desc="picture: source (max-width:500px) broken image, img broken image"
data-narrow="/images/broken.png?70-{{GET[id]}}"
data-wide="/images/broken.png?71-{{GET[id]}}">
</picture>

<picture>
<source media="(max-width:500px)" srcset="/images/broken.png?80-{{GET[id]}}">
<img src="/images/green-2x2.png?81-{{GET[id]}}"
data-desc="picture: source (max-width:500px) broken image, img valid image"
data-narrow="/images/broken.png?80-{{GET[id]}}"
data-wide="/images/green-2x2.png?81-{{GET[id]}}">
</picture>

<picture>
<source media="(max-width:500px)" srcset="/images/green-1x1.png?90-{{GET[id]}}">
<img src="/images/broken.png?91-{{GET[id]}}"
data-desc="picture: source (max-width:500px) valid image, img broken image"
data-narrow="/images/green-1x1.png?90-{{GET[id]}}"
data-wide="/images/broken.png?91-{{GET[id]}}">
</picture>

<picture>
<source media="(max-width:500px)" srcset="/images/green-1x1.png?100-{{GET[id]}}">
<img src="/images/green-2x2.png?101-{{GET[id]}}"
data-desc="picture: source (max-width:500px) valid image, img valid image"
data-narrow="/images/green-1x1.png?100-{{GET[id]}}"
data-wide="/images/green-2x2.png?101-{{GET[id]}}">
</picture>

<picture>
<source media="(max-width:500px)" srcset="/images/green-1x1.png?110-{{GET[id]}}">
<img src="/images/green-1x1.png?110-{{GET[id]}}"
data-desc="picture: same URL in source (max-width:500px) and img"
data-narrow="/images/green-1x1.png?110-{{GET[id]}}"
data-wide="/images/green-1x1.png?110-{{GET[id]}}"
data-no-change>
</picture>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 90,
        "byte_start": 17,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 90,
        "byte_start": 17,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.empty",
      "message": "Bad value “” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 175,
        "byte_start": 92,
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
        "byte_end": 175,
        "byte_start": 92,
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
        "byte_end": 371,
        "byte_start": 177,
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
        "byte_end": 575,
        "byte_start": 373,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “/images/broken.png?50-{{GET[id]}}” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 779,
        "byte_start": 577,
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
        "byte_end": 779,
        "byte_start": 577,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “/images/green-1x1.png?60-{{GET[id]}}” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 991,
        "byte_start": 781,
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
        "byte_end": 991,
        "byte_start": 781,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “/images/broken.png?70-{{GET[id]}}” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1080,
        "byte_start": 1003,
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
        "byte_end": 1297,
        "byte_start": 1081,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “/images/broken.png?80-{{GET[id]}}” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1397,
        "byte_start": 1320,
        "col": 1,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1619,
        "byte_start": 1398,
        "col": 1,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “/images/green-1x1.png?90-{{GET[id]}}” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1722,
        "byte_start": 1642,
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
        "byte_end": 1941,
        "byte_start": 1723,
        "col": 1,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “/images/green-1x1.png?100-{{GET[id]}}” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2045,
        "byte_start": 1964,
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
        "byte_end": 2272,
        "byte_start": 2046,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “/images/green-1x1.png?110-{{GET[id]}}” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2376,
        "byte_start": 2295,
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
        "byte_end": 2609,
        "byte_start": 2377,
        "col": 1,
        "line": 73
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
  "source_name": "html/semantics/embedded-content/the-img-element/environment-changes/iframed.sub.html"
}
```
