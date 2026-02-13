# html/semantics/embedded-content/the-object-element/usemap-casing.html

Counts:
- errors: 7
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/usemap-casing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>object usemap case-sensitive</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/infrastructure.html#rules-for-parsing-a-hash-name-reference">
<!-- See also: https://github.com/whatwg/html/issues/1666 -->

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<object data="/images/threecolors.png" usemap="#sanityCheck" width="100" height="100"></object>
<map name="sanityCheck"><area shape="rect" coords="0,0,100,100"></map>

<object data="/images/threecolors.png" usemap="#sImPlE" width="100" height="100"></object>
<map name="simple"><area shape="rect" coords="0,0,100,100"></map>
<map name="SIMPLE"><area shape="rect" coords="0,0,100,100"></map>

<object data="/images/threecolors.png" usemap="#paSSfield-killroyß" width="100" height="100"></object>
<map name="passfield-killroyß"><area shape="rect" coords="0,0,100,100"></map>
<map name="PASSFIELD-KILLROYß"><area shape="rect" coords="0,0,100,100"></map>
<map name="paſſfield-killroyß"><area shape="rect" coords="0,0,100,100"></map>
<map name="passfield-&#x212a;illroyß"><area shape="rect" coords="0,0,100,100"></map>
<map name="paßfield-killroyß"><area shape="rect" coords="0,0,100,100"></map>
<map name="paẞfield-killroyß"><area shape="rect" coords="0,0,100,100"></map>
<map name="passfield-killroyẞ"><area shape="rect" coords="0,0,100,100"></map>
<map name="passﬁeld-killroyß"><area shape="rect" coords="0,0,100,100"></map>
<map name="passfıeld-killroyß"><area shape="rect" coords="0,0,100,100"></map>
<map name="passfİeld-killroyß"><area shape="rect" coords="0,0,100,100"></map>

<object data="/images/threecolors.png" usemap="#глупый" width="100" height="100"></object>
<map name="глупы&#x438;&#x306;"><area shape="rect" coords="0,0,100,100"></map>
<map name="ГЛУПЫЙ"><area shape="rect" coords="0,0,100,100"></map>
<map name="ГЛУПЫ&#x418;&#x306;"><area shape="rect" coords="0,0,100,100"></map>

<object data="/images/threecolors.png" usemap="#åωk" width="100" height="100"></object>
<map name="ÅΩK"><area shape="rect" coords="0,0,100,100"></map>
<map name="&#x212b;ωk"><area shape="rect" coords="0,0,100,100"></map>
<map name="å&#x2126;k"><area shape="rect" coords="0,0,100,100"></map>
<map name="åω&#x212a;"><area shape="rect" coords="0,0,100,100"></map>

<object data="/images/threecolors.png" usemap="#blah1" width="100" height="100"></object>
<map name="blah&#x2460;"><area shape="rect" coords="0,0,100,100"></map>
<map name="bl&#x24b6;h1"><area shape="rect" coords="0,0,100,100"></map>
<map name="bl&#x24d0;h1"><area shape="rect" coords="0,0,100,100"></map>

<object data="/images/threecolors.png" usemap="#t&Eacute;dz5アパートFi" width="100" height="100"></object>
<map name="T&Eacute;DZ5アパートFi"><area shape="rect" coords="0,0,100,100"></map>
<map name="T&eacute;&#x01F1;&#x2075;アパートFi"><area shape="rect" coords="0,0,100,100"></map>
<map name="t&Eacute;dz5&#x3300;Fi"><area shape="rect" coords="0,0,100,100"></map>
<map name="t&Eacute;dz5&#x30A2;&#x30CF;&#x309A;&#x30FC;&#x30C8;Fi"><area shape="rect" coords="0,0,100,100"></map>
<map name="T&Eacute;DZ⁵アパートFi"><area shape="rect" coords="0,0,100,100"></map>
<map name="T&Eacute;DZ5アパートﬁ"><area shape="rect" coords="0,0,100,100"></map>

<object data="/images/threecolors.png" usemap="#ΣΣ" width="100" height="100"></object>
<map name="σς"><area shape="rect" coords="0,0,100,100"></map>

<div id="log"></div>

<script>
"use strict";
setup({ explicit_done: true });

onload = () => {
  const objects = Array.from(document.querySelectorAll(`object`));

  for (let object of objects) {
    test(() => {
      const objectRect = object.getBoundingClientRect();
      const x = objectRect.left + objectRect.width / 2;
      const y = objectRect.top + objectRect.height / 2;
      const element = document.elementFromPoint(x, y);

      const name = element.parentElement.getAttribute("name");
      const messageSuffix = name ? `; used <map> with name "${name}"` : "";

      assert_equals(element, object, "The element retrieved must be the object, not an area" + messageSuffix);
    }, `Object with usemap of ${object.useMap} should not match any of the areas (it does not support usemap)`);
  }

  done();
};
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “map” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 1221,
        "byte_start": 1182,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “map” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 1882,
        "byte_start": 1845,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “map” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 2038,
        "byte_start": 2001,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “map” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 2265,
        "byte_start": 2241,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “map” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 2336,
        "byte_start": 2312,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “map” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 2408,
        "byte_start": 2383,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “map” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 3208,
        "byte_start": 3141,
        "col": 1,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “sImPlE”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 696,
        "byte_start": 615,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “paSSfield-killroyß”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 933,
        "byte_start": 839,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “глупый”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 1835,
        "byte_start": 1748,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “åωk”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 2166,
        "byte_start": 2086,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “blah1”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 2536,
        "byte_start": 2456,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “tÉdz5アパートFi”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 2864,
        "byte_start": 2763,
        "col": 1,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “ΣΣ”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 3510,
        "byte_start": 3431,
        "col": 1,
        "line": 54
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
  "source_name": "html/semantics/embedded-content/the-object-element/usemap-casing.html"
}
```
