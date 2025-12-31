% use XeLaTeX for rendering
\NeedsTeXFormat{LaTeX2e}
% class name and tag
\ProvidesClass{vitae}[2025/12/31]
% document class
\LoadClass[letterpaper, 11pt]{article}
% package section
\RequirePackage{latexsym}
\RequirePackage[empty]{fullpage}
\RequirePackage{titlesec}
\RequirePackage{marvosym}
\RequirePackage[usenames,dvipsnames]{color}
\RequirePackage{verbatim}
\RequirePackage{enumitem}
\RequirePackage[hidelinks]{hyperref}
\RequirePackage{fancyhdr}
\RequirePackage[english]{babel}
\RequirePackage{tabularx}
\RequirePackage{fontawesome5}
\RequirePackage{ragged2e}
\RequirePackage{etoolbox}
\RequirePackage{tikz}
\RequirePackage{fontspec}
\RequirePackage{xeCJK}

% --- Page Setup ---
\pagestyle{fancy}
\fancyhf{}  
\fancyhf[OFR]{\fontsize{10}{12} \emph{Revised as of November 2025}}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{0pt}

\addtolength{\oddsidemargin}{-0.5in}
\addtolength{\evensidemargin}{-0.5in}
\addtolength{\textwidth}{1in}
\addtolength{\topmargin}{-.5in}
\addtolength{\textheight}{1.0in}

\urlstyle{same}
\raggedbottom
\raggedright
\setlength{\tabcolsep}{0in}
\setlength{\footskip}{5pt}

% --- Section Formatting ---
\titleformat{\section}{
  \vspace{-4pt}\scshape\raggedright\large
}{}{0em}{}[\color{black}\titlerule \vspace{-5pt}]

% --- Custom Macros ---
% Individual bullet point
\newcommand{\cvbullet}[1]{
  \item{
    {#1\vspace{-2pt}}
  }
}
% Heading block: {Title}{Location}{Subtitle}{Date}
\newcommand{\cvheading}[4]{
  \vspace{-7pt}\item\
    \begin{tabular*}{\textwidth}[t]{l@{\extracolsep{\fill}}r}
      \textbf{#1} & #2 \\
      \small #3 & \small#4 \\
    \end{tabular*}\vspace{-6pt}
}

\newcommand{\cvitemstart}{\begin{itemize}[leftmargin=0in, label={}]}
\newcommand{\cvitemend}{\end{itemize}}
\newcommand{\cvbulletstart}{\begin{itemize}\justifying}
\newcommand{\cvbulletend}{\end{itemize}\vspace{-12pt}}
\renewcommand\labelitemii{$\vcenter{\hbox{\footnotesize$\bullet$}}$}