use chrono::{Datelike, Utc};
use leptos::*;

use crate::{components::content::BaseContent, pages::base::BasePage};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <BasePage title="Sobre mim" enable_back_to_top=true>
            <BaseContent
                title="Sobre mim"
                bg_color=Some("#4ce16599".to_string())
                bg_img=Some("url('/img/aboutme.jpg')".to_string())
                back_href="/"
            >
                <AboutContent />
            </BaseContent>
        </BasePage>
    }
}

#[component]
fn AboutContent() -> impl IntoView {
    let age = Utc::now().year() - 1999;

    view! {
        <ul>
            <li>Meu nome é Luiz Junio e tenho {age} anos</li>
            <li>Uso SeraphyBR como meu Nickname na internet, originalmente para usar em MMORPGs.</li>
            <li>
                Minha foto de perfil no github, e algumas outras contas, é uma imagem muito antiga que encontrei na internet,
                se trata do Digimon Dorumon (Um personagem fictício) com suas cores trocadas.
            </li>
        </ul>

        <h2>Sobre este blog</h2>

        <ul>
            <li>Esse blog havia sido originalmente criado para uma atividade de uma disciplina enquanto fazia minha graduação em Ciência da Computação</li>
            <li>
                Seu propósito inicial era para postar coisas que acharia uteis compartilhar, de coisas que aprendi e/ou vi na internet.
                Mas o objetivo principal atualmente é para servir como um portifolio dos meus projetos que compartilho em plataformas como Github e
                contar sobre a experiencia e aprendizado obtido com isso. Esse blog por si só tambem se inclui nisso, pois eu mesmo o desenvolvi sem usar plataformas
                prontas como Medium e outros, justamente para colocar minhas habilidades em prática.
            </li>
            <li>
                A primeira versão desse blog foi criado em meados de 2019 usando <a href="https://jekyllrb.com/">Jekyll</a>, que era bem conhecido na época como
                um bom gerador de site estático que dava suporte a temas e tinha um suporte para ser hospedado de forma simplificada nas plataformas do Github Pages
                e Gitlab Pages, e fazia o trabalho de converter os arquivos markdown nas páginas. Nessa época eu não tinha muito conhecimento de Frontend, mas ao mesmo tempo
                não queria usar algo como Wordpress ou Blogger "(que outros colegas do curso optaram)" porque queria experimentar algo diferente.
                Então foi o que eu fiz, encontrei um tema de template pro Jeklly que se chamava Moon, dei uma leve remodelada em cores, icones e bordas
                para ficar no meu gosto, e criei meus primeiros posts em markdown, a pedido da disciplina do curso.
            </li>
            <li>
                Em 2024, após todas as experiencias que adquiri durante o Curso, estágio e trabalho, eu finalmente dei inicio ao projeto de
                reescrever todo o Blog em tecnologias mais modernas, onde eu teria muito mais controle, entenderia de fato como funciona, me sentiria mais orgulhoso e ainda
                poderia colocar em prática meus conhecimentos em desenvolvimento Web e demais tecnologias, mantendo-se o mais fiel possível
                ao design do meu primeiro Blog com Jeklly, que eu já me sentia confortável e ajudaria no processo.
            </li>
            <li>
                Mais detalhes técnicos sobre o desenvolvimento desse Blog, processo de decisão de escolha das tecnologias usadas, design, e paralelos com o original em Jeklly,
                estarão presentes na parte de Projetos.
            </li>
        </ul>
    }
}
