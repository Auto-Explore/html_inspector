# html/dom/elements/global-attributes/lang-xmllang-01.html

Counts:
- errors: 2
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/lang-xmllang-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Languages</title>
<link rel="match" href="lang-xmllang-01-ref.html">
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-lang-and-xml:lang-attributes">
<link rel="help" href="http://www.w3.org/TR/CSS2/selector.html#lang">
<meta name="flags" content="css21">
<style>
#test #a :lang(en) { background: limegreen; }
#test #b :lang(nl) { background: limegreen; }
#test #c :lang(en) { background: limegreen; }
#test #d :lang(nl) { background: limegreen; }
#test #e :lang(en) { background: limegreen; }
#test #f :lang(en) { background: limegreen; }
#test #g :lang(de) { background: limegreen; }
</style>
<body>
<p>All lines below should have a green background.</p>
<div id="test" lang="nl">
<div id="a"><p lang="en">{}{lang}{en}</p></div>
<div id="b"><p xml:lang="en">{}{xml:lang}{en}</p></div>
<div id="c"><div lang="en"><p>Parent: {}{lang}{en}</p></div></div>
<div id="d"><div xml:lang="en"><p>Parent: {}{xml:lang}{en}</p></div></div>
</div>
<script>
try {
  var XML = "http://www.w3.org/XML/1998/namespace";
  var container = document.getElementById("test");

  var div = document.createElement("div");
  div.id = "e";
  var testNode = document.createElement("p");
  testNode.appendChild(document.createTextNode("{xml}{lang}{en}"));
  testNode.setAttributeNS(XML, "xml:lang", "en");
  div.appendChild(testNode);
  container.appendChild(div);

  div = document.createElement("div");
  div.id = "f";
  testNode = document.createElement("p");
  testNode.appendChild(document.createTextNode("{xml}{lang}{en} - {lang}{de}"));
  testNode.setAttributeNS(XML, "xml:lang", "en");
  testNode.setAttributeNS(null, "lang", "de");
  div.appendChild(testNode);
  container.appendChild(div);

  div = document.createElement("div");
  div.id = "g";
  testNode = document.createElement("p");
  testNode.appendChild(document.createTextNode("{xml}{lang}{de} - {lang}{en}"));
  testNode.setAttributeNS(XML, "xml:lang", "de");
  testNode.setAttributeNS(null, "lang", "en");
  container.appendChild(testNode);
  div.appendChild(testNode);
  container.appendChild(div);
} catch (e) {
}
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.xml_lang.requires_lang",
      "message": "When the attribute “xml:lang” in no namespace is specified, the element must also have the attribute “lang” present with the same value.",
      "severity": "Error",
      "span": {
        "byte_end": 866,
        "byte_start": 849,
        "col": 13,
        "line": 21
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.xml_lang.requires_lang",
      "message": "When the attribute “xml:lang” in no namespace is specified, the element must also have the attribute “lang” present with the same value.",
      "severity": "Error",
      "span": {
        "byte_end": 991,
        "byte_start": 972,
        "col": 13,
        "line": 23
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
  "source_name": "html/dom/elements/global-attributes/lang-xmllang-01.html"
}
```
