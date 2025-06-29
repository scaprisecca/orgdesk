<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta name="generator" content="docs.rs 0.6.0 (ff5ebf09 2025-06-25)"><link rel="stylesheet" href="/-/static/vendored.css?0-6-0-ff5ebf09-2025-06-25" media="all" />
        <link rel="stylesheet" href="/-/static/style.css?0-6-0-ff5ebf09-2025-06-25" media="all" />
        <link rel="stylesheet" href="/-/static/font-awesome.css?0-6-0-ff5ebf09-2025-06-25" media="all" />

        <link rel="search" href="/-/static/opensearch.xml" type="application/opensearchdescription+xml" title="Docs.rs" />

        <title>orgtoical 0.2.1 - Docs.rs</title><script nonce="x9J4CAu+8radVRbwlxYtBaFmY/NvSv0QnurQFP+AhvgC2s1N">(function() {
    function applyTheme(theme) {
        if (theme) {
            document.documentElement.dataset.docsRsTheme = theme;
        }
    }

    window.addEventListener("storage", ev => {
        if (ev.key === "rustdoc-theme") {
            applyTheme(ev.newValue);
        }
    });

    // see ./storage-change-detection.html for details
    window.addEventListener("message", ev => {
        if (ev.data && ev.data.storage && ev.data.storage.key === "rustdoc-theme") {
            applyTheme(ev.data.storage.value);
        }
    });

    applyTheme(window.localStorage.getItem("rustdoc-theme"));
})();</script><script defer type="text/javascript" nonce="x9J4CAu+8radVRbwlxYtBaFmY/NvSv0QnurQFP+AhvgC2s1N" src="/-/static/menu.js?0-6-0-ff5ebf09-2025-06-25"></script>
        <script defer type="text/javascript" nonce="x9J4CAu+8radVRbwlxYtBaFmY/NvSv0QnurQFP+AhvgC2s1N" src="/-/static/index.js?0-6-0-ff5ebf09-2025-06-25"></script>
    </head>

    <body class="flex">
<div class="nav-container">
    <div class="container">
        <div class="pure-menu pure-menu-horizontal" role="navigation" aria-label="Main navigation">
            <form action="/releases/search"
                  method="GET"
                  id="nav-search-form"
                  class="landing-search-form-nav  ">

                
                <a href="/" class="pure-menu-heading pure-menu-link docsrs-logo" aria-label="Docs.rs">
                    <span title="Docs.rs"><span class="fa fa-solid fa-cubes " aria-hidden="true"></span></span>
                    <span class="title">Docs.rs</span>
                </a><ul class="pure-menu-list">
    <script id="crate-metadata" type="application/json">
        
        {
            "name": "orgtoical",
            "version": "0.2.1"
        }
    </script><li class="pure-menu-item">
            <a href="/crate/orgtoical/0.2.1" class="pure-menu-link crate-name" title="Export org-mode files to iCalendar">
                <span class="fa fa-solid fa-cube " aria-hidden="true"></span>
                <span class="title">orgtoical-0.2.1</span>
            </a>
        </li>
    
</ul><div class="spacer"></div>
                
                

<ul class="pure-menu-list">
                    <li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="docs.rs">docs.rs</a>
                        <ul class="pure-menu-children">
                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="/about" >
            About docs.rs
        </a>
    </li>

                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs" target="_blank">
            Privacy policy
        </a>
    </li>

                        </ul>
                    </li>
                </ul>
                <ul class="pure-menu-list"><li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="Rust">Rust</a>
                        <ul class="pure-menu-children">
                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="https://www.rust-lang.org/" target="_blank">
            Rust website
        </a>
    </li>

                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="https://doc.rust-lang.org/book/" target="_blank">
            The Book
        </a>
    </li>


                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="https://doc.rust-lang.org/std/" target="_blank">
            Standard Library API Reference
        </a>
    </li>


                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="https://doc.rust-lang.org/rust-by-example/" target="_blank">
            Rust by Example
        </a>
    </li>


                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="https://doc.rust-lang.org/cargo/guide/" target="_blank">
            The Cargo Guide
        </a>
    </li>


                            
    <li class="pure-menu-item">
        <a class="pure-menu-link" href="https://doc.rust-lang.org/nightly/clippy" target="_blank">
            Clippy Documentation
        </a>
    </li>

                        </ul>
                    </li>
                </ul>
                
                <div id="search-input-nav">
                    <label for="nav-search">
                        <span class="fa fa-solid fa-magnifying-glass " aria-hidden="true"></span>
                    </label>

                    
                    
                    <input id="nav-search" name="query" type="text" aria-label="Find crate by search query" tabindex="-1"
                        placeholder="Find crate"
                        >
                </div>
            </form>
        </div>
    </div>
</div>
    
    <div class="docsrs-package-container">
        <div class="container">
            <div class="description-container">
                

                
                <h1 id="crate-title">
                    orgtoical 0.2.1
                    <span id="clipboard" class="svg-clipboard" title="Copy crate name and version information"></span>
                </h1>

                
                <div class="description">Export org-mode files to iCalendar</div>


                <div class="pure-menu pure-menu-horizontal">
                    <ul class="pure-menu-list">
                        
                        <li class="pure-menu-item"><a href="/crate/orgtoical/0.2.1"
                                class="pure-menu-link">
                                <span class="fa fa-solid fa-cube " aria-hidden="true"></span>
                                <span class="title"> Crate</span>
                            </a>
                        </li>

                        
                        <li class="pure-menu-item">
                            <a href="/crate/orgtoical/0.2.1/source/"
                                class="pure-menu-link pure-menu-active">
                                <span class="fa fa-regular fa-folder-open " aria-hidden="true"></span>
                                <span class="title"> Source</span>
                            </a>
                        </li>

                        
                        <li class="pure-menu-item">
                            <a href="/crate/orgtoical/0.2.1/builds"
                                class="pure-menu-link">
                                <span class="fa fa-solid fa-gears " aria-hidden="true"></span>
                                <span class="title"> Builds</span>
                            </a>
                        </li>

                        
                        <li class="pure-menu-item">
                            <a href="/crate/orgtoical/0.2.1/features"
                               class="pure-menu-link">
                                <span class="fa fa-solid fa-flag " aria-hidden="true"></span>
                                <span class="title">Feature flags</span>
                            </a>
                        </li>
                    </ul>
                </div>
            </div></div>
    </div>

    <div class="container package-page-container small-bottom-pad">
        <div class="pure-g">
            <div id="side-menu" class="pure-u-1 pure-u-sm-7-24 pure-u-md-5-24 source-view">
                <div class="pure-menu package-menu">
                    <ul class="pure-menu-list">
                        
                        
                            <li class="pure-menu-item toggle-source">
                                <button aria-label="Hide source sidebar" title="Hide source sidebar" aria-expanded="true"><span class="left"><span class="fa fa-solid fa-chevron-left " aria-hidden="true"></span></span><span class="right"><span class="fa fa-solid fa-chevron-right " aria-hidden="true"></span></span> <span class="text">Hide files</span></button>
                            </li>
                        
                        <li class="pure-menu-item">
                                <a href="../" class="pure-menu-link"><span class="fa fa-regular fa-folder-open " aria-hidden="true"></span> <span class="text">..</span></a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./ical.rs" class="pure-menu-link">
                                    <span class="fa fa-brands fa-rust " aria-hidden="true"></span>

                                    <span class="text">ical.rs</span>
                                </a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./main.rs" class="pure-menu-link">
                                    <span class="fa fa-brands fa-rust " aria-hidden="true"></span>

                                    <span class="text">main.rs</span>
                                </a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./org.rs" class="pure-menu-link">
                                    <span class="fa fa-brands fa-rust " aria-hidden="true"></span>

                                    <span class="text">org.rs</span>
                                </a>
                            </li></ul>
                </div>
            </div>

            
                
                    
                
                <div id="source-code-container" class="pure-u-1 pure-u-sm-17-24 pure-u-md-19-24">
                    <div data-nosnippet class="source-code"><pre id="line-numbers"><code><a href="#1" id="1">1</a>
<a href="#2" id="2">2</a>
<a href="#3" id="3">3</a>
<a href="#4" id="4">4</a>
<a href="#5" id="5">5</a>
<a href="#6" id="6">6</a>
<a href="#7" id="7">7</a>
<a href="#8" id="8">8</a>
<a href="#9" id="9">9</a>
<a href="#10" id="10">10</a>
<a href="#11" id="11">11</a>
<a href="#12" id="12">12</a>
<a href="#13" id="13">13</a>
<a href="#14" id="14">14</a>
<a href="#15" id="15">15</a>
<a href="#16" id="16">16</a>
<a href="#17" id="17">17</a>
<a href="#18" id="18">18</a>
<a href="#19" id="19">19</a>
<a href="#20" id="20">20</a>
<a href="#21" id="21">21</a>
<a href="#22" id="22">22</a>
<a href="#23" id="23">23</a>
<a href="#24" id="24">24</a>
<a href="#25" id="25">25</a>
<a href="#26" id="26">26</a>
<a href="#27" id="27">27</a>
<a href="#28" id="28">28</a>
<a href="#29" id="29">29</a>
<a href="#30" id="30">30</a>
<a href="#31" id="31">31</a>
<a href="#32" id="32">32</a>
<a href="#33" id="33">33</a>
<a href="#34" id="34">34</a>
<a href="#35" id="35">35</a>
<a href="#36" id="36">36</a>
</code></pre></div>
                    <div id="source-code" class="source-code"><pre><code><span class="syntax-source syntax-rust"><span class="syntax-keyword syntax-other syntax-rust">use</span> <span class="syntax-meta syntax-path syntax-rust">orgize<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-path syntax-rust">rowan<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-path syntax-rust">ast<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span>AstNode<span class="syntax-punctuation syntax-terminator syntax-rust">;</span>
<span class="syntax-keyword syntax-other syntax-rust">use</span> <span class="syntax-meta syntax-path syntax-rust">std<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-path syntax-rust">io<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-begin syntax-rust">{</span><span class="syntax-variable syntax-language syntax-rust">self</span><span class="syntax-punctuation syntax-separator syntax-rust">,</span> Read</span><span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-end syntax-rust">}</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

<span class="syntax-meta syntax-module syntax-rust"><span class="syntax-storage syntax-type syntax-module syntax-rust">mod</span> <span class="syntax-entity syntax-name syntax-module syntax-rust">org</span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span></span>
<span class="syntax-keyword syntax-other syntax-rust">use</span> <span class="syntax-meta syntax-path syntax-rust">org<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-keyword syntax-operator syntax-arithmetic syntax-rust">*</span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

<span class="syntax-meta syntax-module syntax-rust"><span class="syntax-storage syntax-type syntax-module syntax-rust">mod</span> <span class="syntax-entity syntax-name syntax-module syntax-rust">ical</span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span></span>
<span class="syntax-keyword syntax-other syntax-rust">use</span> <span class="syntax-meta syntax-path syntax-rust">ical<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-keyword syntax-operator syntax-arithmetic syntax-rust">*</span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

<span class="syntax-meta syntax-function syntax-rust"><span class="syntax-meta syntax-function syntax-rust"><span class="syntax-storage syntax-type syntax-function syntax-rust">fn</span> </span><span class="syntax-entity syntax-name syntax-function syntax-rust">main</span></span><span class="syntax-meta syntax-function syntax-rust"><span class="syntax-meta syntax-function syntax-parameters syntax-rust"><span class="syntax-punctuation syntax-section syntax-parameters syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-function syntax-rust"><span class="syntax-meta syntax-function syntax-parameters syntax-rust"><span class="syntax-punctuation syntax-section syntax-parameters syntax-end syntax-rust">)</span></span></span></span><span class="syntax-meta syntax-function syntax-rust"> <span class="syntax-meta syntax-function syntax-return-type syntax-rust"><span class="syntax-punctuation syntax-separator syntax-rust">-&gt;</span> <span class="syntax-meta syntax-path syntax-rust">io<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-generic syntax-rust"><span class="syntax-support syntax-type syntax-rust">Result</span><span class="syntax-punctuation syntax-definition syntax-generic syntax-begin syntax-rust">&lt;</span><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span><span class="syntax-punctuation syntax-definition syntax-generic syntax-end syntax-rust">&gt;</span></span></span> </span><span class="syntax-meta syntax-function syntax-rust"><span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-begin syntax-rust">{</span>
    <span class="syntax-storage syntax-type syntax-rust">let</span> <span class="syntax-storage syntax-modifier syntax-rust">mut</span> org_string <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> <span class="syntax-support syntax-type syntax-rust">String</span><span class="syntax-meta syntax-path syntax-rust"><span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span>new<span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>
    <span class="syntax-meta syntax-path syntax-rust">std<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-path syntax-rust">io<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span>stdin<span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">read_to_string</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span><span class="syntax-keyword syntax-operator syntax-bitwise syntax-rust">&amp;</span><span class="syntax-storage syntax-modifier syntax-rust">mut</span> org_string</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-keyword syntax-operator syntax-rust">?</span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

    <span class="syntax-storage syntax-type syntax-rust">let</span> <span class="syntax-storage syntax-modifier syntax-rust">mut</span> reminder <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> <span class="syntax-support syntax-type syntax-rust">None</span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

    <span class="syntax-storage syntax-type syntax-rust">let</span> <span class="syntax-storage syntax-modifier syntax-rust">mut</span> args <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> <span class="syntax-meta syntax-path syntax-rust">std<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-path syntax-rust">env<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span>args<span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>
    <span class="syntax-keyword syntax-control syntax-rust">while</span> <span class="syntax-storage syntax-type syntax-rust">let</span> <span class="syntax-support syntax-type syntax-rust">Some</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span>arg</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span> <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> args<span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">next</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span> <span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-begin syntax-rust">{</span>
        <span class="syntax-keyword syntax-control syntax-rust">if</span> arg<span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">as_str</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span> <span class="syntax-keyword syntax-operator syntax-comparison syntax-rust">==</span> <span class="syntax-string syntax-quoted syntax-double syntax-rust"><span class="syntax-punctuation syntax-definition syntax-string syntax-begin syntax-rust">&quot;</span>--reminder<span class="syntax-punctuation syntax-definition syntax-string syntax-end syntax-rust">&quot;</span></span> <span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-begin syntax-rust">{</span>
            reminder <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> args<span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">next</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>
        </span><span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-end syntax-rust">}</span></span>
    </span><span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-end syntax-rust">}</span></span>

    <span class="syntax-storage syntax-type syntax-rust">let</span> org_doc <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> <span class="syntax-meta syntax-path syntax-rust">orgize<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-path syntax-rust">Org<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span>parse<span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span><span class="syntax-keyword syntax-operator syntax-bitwise syntax-rust">&amp;</span>org_string</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>
    <span class="syntax-storage syntax-type syntax-rust">let</span> events <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> <span class="syntax-support syntax-function syntax-rust">extract_events</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span>org_doc<span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">document</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">syntax</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span>
        <span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">iter</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span>
        <span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">filter_map</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span>process_event</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span>
        <span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">map</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span><span class="syntax-meta syntax-function syntax-closure syntax-rust"><span class="syntax-meta syntax-function syntax-parameters syntax-rust"><span class="syntax-punctuation syntax-section syntax-parameters syntax-begin syntax-rust">|</span></span></span><span class="syntax-meta syntax-function syntax-closure syntax-rust"><span class="syntax-meta syntax-function syntax-parameters syntax-rust"><span class="syntax-variable syntax-parameter syntax-rust">e</span><span class="syntax-punctuation syntax-section syntax-parameters syntax-end syntax-rust">|</span></span> </span><span class="syntax-meta syntax-function syntax-closure syntax-rust"><span class="syntax-support syntax-function syntax-rust">event_to_ical</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span>e<span class="syntax-punctuation syntax-separator syntax-rust">,</span> <span class="syntax-keyword syntax-operator syntax-bitwise syntax-rust">&amp;</span>reminder</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span></span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span>
        <span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-meta syntax-path syntax-rust">collect<span class="syntax-punctuation syntax-accessor syntax-rust">::</span></span><span class="syntax-meta syntax-generic syntax-rust"><span class="syntax-punctuation syntax-definition syntax-generic syntax-begin syntax-rust">&lt;</span><span class="syntax-meta syntax-generic syntax-rust"><span class="syntax-support syntax-type syntax-rust">Vec</span><span class="syntax-punctuation syntax-definition syntax-generic syntax-begin syntax-rust">&lt;</span><span class="syntax-support syntax-type syntax-rust">String</span><span class="syntax-punctuation syntax-definition syntax-generic syntax-end syntax-rust">&gt;</span></span><span class="syntax-punctuation syntax-definition syntax-generic syntax-end syntax-rust">&gt;</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span>
        <span class="syntax-punctuation syntax-accessor syntax-dot syntax-rust">.</span><span class="syntax-support syntax-function syntax-rust">join</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span><span class="syntax-string syntax-quoted syntax-double syntax-rust"><span class="syntax-punctuation syntax-definition syntax-string syntax-begin syntax-rust">&quot;</span><span class="syntax-constant syntax-character syntax-escape syntax-rust">\n</span><span class="syntax-punctuation syntax-definition syntax-string syntax-end syntax-rust">&quot;</span></span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

    <span class="syntax-storage syntax-type syntax-rust">let</span> calendar <span class="syntax-keyword syntax-operator syntax-assignment syntax-rust">=</span> <span class="syntax-support syntax-function syntax-rust">wrap_calendar</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span><span class="syntax-keyword syntax-operator syntax-bitwise syntax-rust">&amp;</span>events</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

    <span class="syntax-support syntax-macro syntax-rust">print!</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-string syntax-quoted syntax-double syntax-rust"><span class="syntax-punctuation syntax-definition syntax-string syntax-begin syntax-rust">&quot;</span><span class="syntax-constant syntax-other syntax-placeholder syntax-rust">{}</span><span class="syntax-punctuation syntax-definition syntax-string syntax-end syntax-rust">&quot;</span></span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-separator syntax-rust">,</span> calendar<span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span><span class="syntax-punctuation syntax-terminator syntax-rust">;</span>

    <span class="syntax-support syntax-type syntax-rust">Ok</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-begin syntax-rust">(</span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span></span><span class="syntax-meta syntax-group syntax-rust"><span class="syntax-punctuation syntax-section syntax-group syntax-end syntax-rust">)</span></span>
</span><span class="syntax-meta syntax-block syntax-rust"><span class="syntax-punctuation syntax-section syntax-block syntax-end syntax-rust">}</span></span></span>
</span></code></pre></div>
                </div></div>
    </div>
        <script nonce="x9J4CAu+8radVRbwlxYtBaFmY/NvSv0QnurQFP+AhvgC2s1N" type="text/javascript" src="/-/static/source.js?0-6-0-ff5ebf09-2025-06-25"></script>
    </body>
</html>