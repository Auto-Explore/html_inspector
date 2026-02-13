# html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-applets.html

Counts:
- errors: 0
- warnings: 11
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-applets.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.applets</title>
<link rel="help" href="https://html.spec.whatwg.org/C/#dom-document-applets">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<object></object>
<object type="application/x-java-applet"></object>
<object type="application/x-java-bean"></object>
<object type="application/x-java-vm"></object>
<object><param name="type" value="application/x-java-applet" /></object>
<object><param name="type" value="application/x-java-bean" /></object>
<object><param name="type" value="application/x-java-vm" /></object>

<script>
test(() => {
  const collection = document.applets;
  assert_true(collection instanceof HTMLCollection);
  assert_equals(collection.length, 0, 'The collection should be empty.');
}, 'document.applets should return an empty collection.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 241,
        "byte_start": 233,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 292,
        "byte_start": 251,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 341,
        "byte_start": 302,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 388,
        "byte_start": 351,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 406,
        "byte_start": 398,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 461,
        "byte_start": 406,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 479,
        "byte_start": 471,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 532,
        "byte_start": 479,
        "col": 9,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 550,
        "byte_start": 542,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 601,
        "byte_start": 550,
        "col": 9,
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
  "source_name": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-applets.html"
}
```
