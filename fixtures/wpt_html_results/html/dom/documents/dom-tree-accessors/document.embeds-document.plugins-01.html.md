# html/dom/documents/dom-tree-accessors/document.embeds-document.plugins-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.embeds-document.plugins-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.embeds and document.plugins</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-embeds">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-plugins">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  assert_equals(document.embeds, document.embeds,
                "embeds should be constant");
  assert_equals(document.plugins, document.plugins,
                "plugins should be constant");
  assert_equals(document.embeds, document.plugins,
                "embeds should be the same as plugins");
  assert_equals(document.embeds.length, 0);
  assert_equals(document.plugins.length, 0);
}, "No plugins");

test(function() {
  var embed = document.body.appendChild(document.createElement("embed"));
  this.add_cleanup(function() { document.body.removeChild(embed) });

  assert_array_equals(document.embeds, [embed]);
  assert_array_equals(document.plugins, [embed]);

  assert_equals(document.embeds, document.embeds,
                "embeds should be constant");
  assert_equals(document.plugins, document.plugins,
                "plugins should be constant");
  assert_equals(document.embeds, document.plugins,
                "embeds should be the same as plugins");
}, "One plugin");

test(function() {
  var embed1 = document.createElement("embed"),
      embed2 = document.createElement("embed");

  document.body.appendChild(embed2);
  this.add_cleanup(function() { document.body.removeChild(embed2) });
  document.body.insertBefore(embed1, embed2);
  this.add_cleanup(function() { document.body.removeChild(embed1) });

  assert_array_equals(document.embeds, [embed1, embed2]);
  assert_array_equals(document.plugins, [embed1, embed2]);

  assert_equals(document.embeds, document.embeds,
                "embeds should be constant");
  assert_equals(document.plugins, document.plugins,
                "plugins should be constant");
  assert_equals(document.embeds, document.plugins,
                "embeds should be the same as plugins");
}, "Two plugins");

test(function() {
  var embed1 = document.createElement("embed"),
      embed2 = document.createElement("embed");
  document.body.appendChild(embed1);
  this.add_cleanup(function() { document.body.removeChild(embed1) });
  var embeds = document.embeds;
  assert_true(embeds instanceof HTMLCollection);
  assert_equals(embeds.length, 1);

  document.body.appendChild(embed2);
  assert_equals(embeds.length, 2);

  document.body.removeChild(embed2);
  assert_equals(embeds.length, 1);
}, "Document.embeds should be a live collection");

test(function() {
  var embed1 = document.createElement("embed"),
      embed2 = document.createElement("embed");
  document.body.appendChild(embed1);
  this.add_cleanup(function() { document.body.removeChild(embed1) });
  var pls = document.plugins;
  assert_true(pls instanceof HTMLCollection);
  assert_equals(pls.length, 1);

  document.body.appendChild(embed2);
  assert_equals(pls.length, 2);

  document.body.removeChild(embed2);
  assert_equals(pls.length, 1);
}, "Document.plugins should be a live collection");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/documents/dom-tree-accessors/document.embeds-document.plugins-01.html"
}
```
