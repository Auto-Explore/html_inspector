# html/semantics/grouping-content/the-ol-element/grouping-ol.html

Counts:
- errors: 5
- warnings: 13
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>the ol element</title>
    <link rel="author" title="dzenana" href="mailto:dzenana.trenutak@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-ol-element">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <h1>Description</h1>
    <p>This test validates the ol element.</p>

    <div id="log"></div>

    <p>Ordered List</p>
    <ol id="basic">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="allAtts" reversed start="3" type="A">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="justRev" reversed>
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="basicRevGoodName" reversed="reversed">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="basicRevNameWithSpace" reversed=" reversed ">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="basicRevEmpty" reversed="" start="A">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="basicRevTrue" reversed="true">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="basicRevFalse" reversed="false">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="start2" start="2">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="negative" start="-10">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="posFloatDown" start="4.03">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="negFloatDown" start="-4.03">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="posFloatUp" start="4.9">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="negFloatUp" start="-4.9">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="exponent" start="7e2">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="decimal" start=".5">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="letter" start="A">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="middle50">
        <li>list item</li>
        <li value="50">list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="middleneg50">
        <li>list item</li>
        <li value="-50">list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="lots" reversed="reversed">
        <li value="10">list item</li>
        <li value="20">list item</li>
<a></a><abbr></abbr><address></address><area></area><article></article><aside></aside><audio></audio><b></b><base></base><bdi></bdi><bdo></bdo><blockquote></blockquote><body></body><br></br><button></button><canvas></canvas><caption></caption><cite></cite><code></code><col></col><colgroup></colgroup><command></command><datalist></datalist><dd></dd><del></del><details></details><dfn></dfn><dialog></dialog><div></div><dl></dl><dt></dt><em></em><embed></embed><fieldset></fieldset><figcaption></figcaption><figure></figure><footer></footer><form></form><h1></h1><h2></h2><h3></h3><h4></h4><h5></h5><h6></h6><head></head><header></header><hgroup></hgroup><hr></hr><html></html><i></i><iframe></iframe><img></img><input></input><ins></ins><kbd></kbd><keygen></keygen><label></label><legend></legend><link></link><map></map><mark></mark><menu></menu><meta></meta><meter></meter><nav></nav><noscript></noscript><object></object><ol><li></li><li></li></ol><optgroup></optgroup><option></option><output></output><p></p><param></param><pre></pre><progress></progress><q></q><rp></rp><rt></rt><ruby></ruby><s></s><samp></samp><script></script><section></section><select></select><small></small><source></source><span></span><strong></strong><style></style><sub></sub><summary></summary><sup></sup><table></table><tbody></tbody><td></td><textarea></textarea><tfoot></tfoot><th></th><thead></thead><time></time><title></title><tr></tr><track></track><u></u><ul><li></li><li></li></ul><var></var><video></video><wbr></wbr>
        <li value="-99">list item</li>
    </ol>

    <script>
        "use strict";

        var testList;

        // check that prototype matches spec's DOM interface
        test(function () {
            testList = document.getElementById("basic");
            assert_equals(Object.getPrototypeOf(testList), HTMLOListElement.prototype, "HTMLOListElement.prototype should be used for OL");
        }, "The prototype for OL is HTMLOListElement.prototype");

        // check that "own" properties reversed, start, and type are present
        test(function () {
            testList = document.getElementById("basic");
            assert_idl_attribute(testList, "reversed");
        }, "'reversed' property should be defined on OL.");

        test(function () {
            testList = document.getElementById("basic");
            assert_idl_attribute(testList, "start");
        }, "'start' property should be defined on OL.");

        test(function () {
            testList = document.getElementById("basic");
            assert_idl_attribute(testList, "type");
        }, "'type' property should be defined on OL.");

        // "The reversed, start, and type IDL attributes must reflect the respective content attributes of the same name."
        test(function () {
            testList = document.getElementById("allAtts");
            assert_true(testList.reversed);
        }, "OL's 'reversed' IDL property reflects content attribute.");

        test(function () {
            testList = document.getElementById("allAtts");
            assert_equals(testList.start, 3);
        }, "OL's 'start' IDL property reflects content attribute.");

        test(function () {
            testList = document.getElementById("allAtts");
            assert_equals(testList.type, "A");
        }, "OL's 'type' IDL property reflects content attribute.");


        // "The reversed attribute is a boolean attribute."

        // check lists for which reversed value should be false
        test(function() {
            assert_false(document.getElementById("basic").reversed, "IDL 'reversed' attribute value false when content attribute absent");
        }, "IDL 'reversed' attribute value false when content attribute absent");

        // check lists for which reversed value should be true
        test(function() {
            assert_true(document.getElementById("justRev").reversed);
            assert_true(document.getElementById("basicRevGoodName").reversed);
            assert_true(document.getElementById("basicRevNameWithSpace").reversed);
            assert_true(document.getElementById("basicRevEmpty").reversed);
            assert_true(document.getElementById("basicRevTrue").reversed);
            assert_true(document.getElementById("basicRevFalse").reversed);
        }, "IDL 'reversed' attribute value true when content attribute exists");

        // check that IDL property works to change reversed value
        test(function() {
            document.getElementById("justRev").reversed = false;
            assert_false(document.getElementById("justRev").reversed, "Changing IDL 'reversed' property changes list's reversed property.");
        }, "Changing IDL 'reversed' property changes list's reversed property.");


        // If the start attribute is present, user agents must parse it as an integer, in order to determine the attribute's value.
        // The default value, used if the attribute is missing or
        //                         if the value cannot be converted to a number according to the referenced algorithm,
        // is 1 if the element has no reversed attribute, and
        // is the number of child li elements otherwise."
        // "The start IDL attribute has the same default as its content attribute."

        test(function() {
            assert_equals(document.getElementById("basic").start, 1);
        }, "Default start value for non-reversed list should be 1");

        test(function() {
            assert_equals(document.getElementById("decimal").start, 1);
        }, "IDL and content attribute parse start of '.5' correctly.");

        test(function() {
            assert_equals(document.getElementById("letter").start, 1);
        }, "IDL and content attribute parse start of 'A' correctly.");

        test(function() {
            assert_equals(document.getElementById("basicRevGoodName").start, 1);
        }, "Default start value (if none provided) for reversed list = 1.");

        test(function() {
            assert_equals(document.getElementById("basicRevEmpty").start, 1);
        }, "Default start value (if failed to parse) for reversed list = 1.");

        test(function() {
            assert_equals(document.getElementById("lots").start, 1);
        }, "Default start value for reversed list = 1 (even with tons of other child elements).");

        test(function() {
            var myList = document.getElementById("basicRevGoodName"), myLI = document.createElement("li");
            myList.appendChild(myLI);
            assert_equals(document.getElementById("basicRevGoodName").start, 1);
        }, "Adding child element to reversed list does not change start value");

        test(function() {
            var myList = document.getElementById("basicRevTrue");
            myList.removeChild(myList.children[0]);
            assert_equals(document.getElementById("basicRevTrue").start, 1);
        }, "Deleting child element from reversed list does not change start value");

        test(function() {
            assert_equals(document.getElementById("start2").start, 2);
        }, "IDL and content attribute parse start of '2' correctly.");

        test(function() {
            assert_equals(document.getElementById("negative").start, -10);
        }, "IDL and content attribute parse start of '-10' correctly.");

        test(function() {
            assert_equals(document.getElementById("posFloatDown").start, 4);
        }, "IDL and content attribute parse start of '4.03' correctly.");

        test(function() {
            assert_equals(document.getElementById("negFloatDown").start, -4);
        }, "IDL and content attribute parse start of '-4.03' correctly.");

        test(function() {
            assert_equals(document.getElementById("posFloatUp").start, 4);
        }, "IDL and content attribute parse start of '4.9' correctly.");

        test(function() {
            assert_equals(document.getElementById("negFloatUp").start, -4);
        }, "IDL and content attribute parse start of '-4.9' correctly.");

        test(function() {
            assert_equals(document.getElementById("exponent").start, 7);
        }, "IDL and content attribute parse start of '7e2' correctly.");

    </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “A” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1347,
        "byte_start": 1302,
        "col": 5,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “4.03” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2121,
        "byte_start": 2086,
        "col": 5,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “-4.03” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2278,
        "byte_start": 2242,
        "col": 5,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “4.9” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2431,
        "byte_start": 2399,
        "col": 5,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “-4.9” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2585,
        "byte_start": 2552,
        "col": 5,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “7e2” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2736,
        "byte_start": 2706,
        "col": 5,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “.5” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2885,
        "byte_start": 2857,
        "col": 5,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “A” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3032,
        "byte_start": 3006,
        "col": 5,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 3613,
        "byte_start": 3607,
        "col": 40,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “area”.",
      "severity": "Error",
      "span": {
        "byte_end": 3620,
        "byte_start": 3613,
        "col": 46,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 3639,
        "byte_start": 3629,
        "col": 62,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.base.not_in_body",
      "message": "Element “base” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3682,
        "byte_start": 3676,
        "col": 109,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.base.missing_href_or_target",
      "message": "Element “base” is missing one or more of the following attributes: “href”, “target”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3682,
        "byte_start": 3676,
        "col": 109,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 3682,
        "byte_start": 3676,
        "col": 109,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “base”.",
      "severity": "Error",
      "span": {
        "byte_end": 3689,
        "byte_start": 3682,
        "col": 115,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.bdo.dir.missing",
      "message": "Element “bdo” must have attribute “dir”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3705,
        "byte_start": 3700,
        "col": 133,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 3742,
        "byte_start": 3736,
        "col": 169,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 3742,
        "byte_start": 3736,
        "col": 169,
        "line": 154
      }
    }
  ],
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol.html"
}
```
