# html/semantics/forms/the-input-element/radio-groupname-case.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/radio-groupname-case.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>radio group name case-sensitive</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#radio-button-group">
<!-- See also: https://github.com/whatwg/html/issues/1666 -->

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<input id=r1 type="radio" name="sImPlE">
<input id=r2 type="radio" name="simple">
<input id=r3 type="radio" name="SIMPLE">

<input id=r4 type="radio" name="paSSfield-killroyß">
<input id=r5 type="radio" name="passfield-killroyß">
<input id=r6 type="radio" name="PASSFIELD-KILLROYß">
<input id=r7 type="radio" name="paſſfield-killroyß">
<input id=r8 type="radio" name="passfield-&#x212a;illroyß">
<input id=r9 type="radio" name="paßfield-killroyß">
<input id=r10 type="radio" name="paẞfield-killroyß">
<input id=r11 type="radio" name="passfield-killroyẞ">
<input id=r12 type="radio" name="passﬁeld-killroyß">
<input id=r13 type="radio" name="passfıeld-killroyß">
<input id=r14 type="radio" name="passfİeld-killroyß">

<input id=r15 type="radio" name="глупый">
<input id=r16 type="radio" name="глупы&#x438;&#x306;">
<input id=r17 type="radio" name="ГЛУПЫЙ">
<input id=r18 type="radio" name="ГЛУПЫ&#x418;&#x306;">

<input id=r19 type="radio" name="åωk">
<input id=r20 type="radio" name="ÅΩK">
<input id=r21 type="radio" name="&#x212b;ωk">
<input id=r22 type="radio" name="å&#x2126;k">
<input id=r23 type="radio" name="åω&#x212a;">

<input id=r24 type="radio" name="blah1">
<input id=r25 type="radio" name="blah&#x2460;">
<input id=r26 type="radio" name="bl&#x24b6;h1">
<input id=r27 type="radio" name="bl&#x24d0;h1">

<input id=r28 type="radio" name="t&Eacute;dz5アパートFi">
<input id=r29 type="radio" name="T&Eacute;DZ5アパートFi">
<input id=r30 type="radio" name="T&eacute;&#x01F1;&#x2075;アパートFi">
<input id=r31 type="radio" name="t&Eacute;dz5&#x3300;Fi">
<input id=r32 type="radio" name="t&Eacute;dz5&#x30A2;&#x30CF;&#x309A;&#x30FC;&#x30C8;Fi">
<input id=r34 type="radio" name="T&Eacute;DZ⁵アパートFi">
<input id=r35 type="radio" name="T&Eacute;DZ5アパートﬁ">

<input id=r36 type="radio" name="ΣΣ">
<input id=r37 type="radio" name="σς">

<script>
"use strict";
const notGroups = {
  "sImPlE": ["r1" ,"r2", "r3"],
  "paSSfield-killroyß": ["r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "r13", "r14"],
  "глупый": ["r15", "r16", "r17", "r18"],
  "åωk": ["r19", "r20", "r21", "r22", "r23"],
  "blah1": ["r24", "r25", "r26", "r27"],
  "tÉdz5アパートFi": ["r28", "r29", "r30", "r31", "r32", "r34", "r35"],
  "ΣΣ": ["r36", "r37"]
};

for (let notGroupLabel of Object.keys(notGroups)) {
  test(() => {
    const ids = notGroups[notGroupLabel];
    const radios = ids.map(id => document.getElementById(id));

    for (let radio of radios) {
      radio.checked = true;
    }

    for (let radio of radios) {
      assert_true(radio.checked, `${radio.name} must be checked`);
    }
  }, `Among names like ${notGroupLabel}, everything must be checkable at the same time`);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “input” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 913,
        "byte_start": 853,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “input” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 1356,
        "byte_start": 1297,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “input” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 1464,
        "byte_start": 1405,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “input” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 1594,
        "byte_start": 1548,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “input” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 1641,
        "byte_start": 1595,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “input” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 1689,
        "byte_start": 1642,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “name” on element “input” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 2223,
        "byte_start": 2134,
        "col": 1,
        "line": 50
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
  "source_name": "html/semantics/forms/the-input-element/radio-groupname-case.html"
}
```
