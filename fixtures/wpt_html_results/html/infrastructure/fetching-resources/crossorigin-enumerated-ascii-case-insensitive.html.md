# html/infrastructure/fetching-resources/crossorigin-enumerated-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 14
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/fetching-resources/crossorigin-enumerated-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#cors-settings-attribute">
<link rel="help" href="https://html.spec.whatwg.org/#enumerated-attribute">
<meta name="assert" content="@crossorigin values are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<img crossorigin="use-credentials">
<img crossorigin="UsE-cReDenTiAlS">
<img crossorigin="uſe-credentialſ">
<img crossorigin="anonymous">
<img crossorigin="AnOnYmOuS">
<img crossorigin="anonymouſ">
<script>
const img = document.querySelectorAll("img");

test(() => {
  assert_equals(img[0].crossOrigin, "use-credentials", "lowercase valid");
  assert_equals(img[1].crossOrigin, "use-credentials", "mixed case valid");
  assert_equals(img[2].crossOrigin, "anonymous", "non-ASCII invalid");
}, "keyword use-credentials");

test(() => {
  assert_equals(img[3].crossOrigin, "anonymous", "lowercase valid");

  // vacuous: the invalid value default is currently anonymous, so even if the
  // UA treats this as invalid, the observable behaviour would still be correct
  assert_equals(img[4].crossOrigin, "anonymous", "mixed case valid");

  // vacuous: the invalid value default is currently anonymous, so even if the
  // UA treats this as valid, the observable behaviour would still be correct
  assert_equals(img[5].crossOrigin, "anonymous", "non-ASCII invalid");
}, "keyword anonymous");
</script>
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
        "byte_end": 413,
        "byte_start": 378,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 413,
        "byte_start": 378,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 449,
        "byte_start": 414,
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
        "byte_end": 449,
        "byte_start": 414,
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
        "byte_end": 487,
        "byte_start": 450,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 487,
        "byte_start": 450,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 517,
        "byte_start": 488,
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
        "byte_end": 517,
        "byte_start": 488,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 547,
        "byte_start": 518,
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
        "byte_end": 547,
        "byte_start": 518,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 578,
        "byte_start": 548,
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
        "byte_end": 578,
        "byte_start": 548,
        "col": 1,
        "line": 13
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
  "source_name": "html/infrastructure/fetching-resources/crossorigin-enumerated-ascii-case-insensitive.html"
}
```
