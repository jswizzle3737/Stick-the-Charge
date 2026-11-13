//! The charge list.
//!
//! The names, section numbers and penalty ranges below are sample content written in the
//! style of a Canadian criminal code listing. They are illustrative, not legal advice.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Summary,
    Hybrid,
    Indictable,
    Varies,
}

// These sample classifications are retained for future review, but are not shown in the UI.
#[allow(dead_code)]
impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Summary => "Summary",
            Kind::Hybrid => "Hybrid",
            Kind::Indictable => "Indictable",
            Kind::Varies => "Varies",
        }
    }

    /// Short plain-language explanation of the classification.
    pub fn gloss(self) -> &'static str {
        match self {
            Kind::Summary => "Less serious matter, tried in provincial court.",
            Kind::Hybrid => "Prosecutor chooses summary or indictable procedure.",
            Kind::Indictable => "Serious matter with the longest available penalties.",
            Kind::Varies => "Procedure and penalties depend on the facts and the applicable law.",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Category {
    Violent,
    Property,
    Weapons,
    Drugs,
    Driving,
    PublicOrder,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::Violent => "Violent",
            Category::Property => "Property",
            Category::Weapons => "Weapons",
            Category::Drugs => "Drugs",
            Category::Driving => "Driving",
            Category::PublicOrder => "Public order",
        }
    }

    /// Stable two-letter tag for this category.
    #[allow(dead_code)]
    pub fn code(self) -> &'static str {
        match self {
            Category::Violent => "VI",
            Category::Property => "PR",
            Category::Weapons => "WE",
            Category::Drugs => "DR",
            Category::Driving => "DV",
            Category::PublicOrder => "PO",
        }
    }

    pub const ALL: [Category; 6] = [
        Category::Violent,
        Category::Property,
        Category::Weapons,
        Category::Drugs,
        Category::Driving,
        Category::PublicOrder,
    ];
}

#[derive(Clone, Copy)]
#[allow(dead_code)] // The illustrative penalties and classifications are deliberately hidden.
pub struct Charge {
    pub number: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub kind: Kind,
    pub penalty: &'static str,
    pub plain: &'static str,
}

pub struct Breakdown {
    pub overview: &'static str,
    pub checks: &'static [&'static str],
    pub limits: &'static str,
}

/// The standard layout: plain explanation, penalty, then the sourced law.
pub struct ChargeGuide {
    pub introduction: &'static str,
    pub points: &'static [&'static str],
    pub context: &'static str,
    pub penalty_section: &'static str,
    pub minimum: &'static str,
    pub penalties: &'static [&'static str],
    pub penalty_note: &'static str,
    pub law_sections: &'static [&'static str],
}

#[path = "additional_guides.rs"]
mod additional_guides;

impl Charge {
    /// The offence and its own punishment, without extra supporting provisions.
    pub fn displayed_law_sections(&self) -> Vec<&'static str> {
        let primary = self.number.split('(').next().unwrap_or(self.number);
        let related = self.related_section().and_then(|(label, section)|
            matches!(label, "Definition" | "Punishment").then_some(section));
        let guide = self.guide();
        let mut sections = Vec::new();
        if let Some(ref guide) = guide {
            for &section in guide.law_sections {
                if (section == primary || section == guide.penalty_section || Some(section) == related)
                    && !sections.contains(&section) {
                    sections.push(section);
                }
            }
        }
        if !sections.contains(&primary) { sections.push(primary); }
        if let Some(section) = related {
            if !sections.contains(&section) { sections.push(section); }
        }
        sections
    }

    pub fn guide(&self) -> Option<ChargeGuide> {
        match (self.category, self.number) {
            (Category::PublicOrder, "46(1)") => Some(ChargeGuide {
                introduction: "High treason includes any of the following acts committed in Canada:",
                points: &[
                    "Killing or attempting to kill the Sovereign, causing the specified bodily harm, maiming, wounding, imprisoning or restraining the Sovereign.",
                    "Levying war against Canada or doing an act in preparation for it.",
                    "Helping an enemy at war with Canada or armed forces fighting Canadian Forces, whether or not a state of war exists with that country.",
                ],
                context: "Section 46 also applies outside Canada to Canadian citizens and persons who owe allegiance to the Sovereign in right of Canada. Section 47 sets a corroboration rule when the evidence comes from only one witness.",
                penalty_section: "47",
                minimum: "Life imprisonment, which section 47 treats as a minimum punishment.",
                penalties: &["Indictable offence: the prescribed sentence is life imprisonment."],
                penalty_note: "Section 47 sets no fine amount. For an individual, section 734 addresses fines when punishment includes a minimum term of imprisonment.",
                law_sections: &["46", "47", "734"],
            }),
            (Category::PublicOrder, "46(2)") => Some(ChargeGuide {
                introduction: "Treason includes several distinct acts committed in Canada:",
                points: &[
                    "Using force or violence to overthrow the government of Canada or a province.",
                    "Without lawful authority, communicating or making available specified military or scientific information to a foreign state's agent, knowing or having reason to know it may be used against Canada's safety or defence.",
                    "Conspiring to commit high treason or the force offence, or forming that intention and showing it by an overt act.",
                    "Conspiring to commit the information offence, or forming that intention and showing it by an overt act.",
                ],
                context: "Section 46 extends to Canadian citizens and persons owing allegiance to the Sovereign in right of Canada outside Canada. Sections 47 and 48 set out a corroboration rule and specific time limits; the speech rule applies to the particular overt acts described there.",
                penalty_section: "47",
                minimum: "Section 47(2) sets no minimum.",
                penalties: &[
                    "Force, high-treason conspiracy or specified intention: indictable offence, up to life imprisonment.",
                    "Specified information offence or related conspiracy or intention: up to life imprisonment during a state of war, or 14 years when no state of war exists.",
                ],
                penalty_note: "Section 47 does not specify a fine amount. The exact paragraph and circumstances matter.",
                law_sections: &["46", "47", "48"],
            }),
            (Category::PublicOrder, "50") => Some(ChargeGuide {
                introduction: "Section 50 covers two different kinds of conduct:",
                points: &[
                    "Inciting or wilfully helping a subject of a state at war with Canada, or whose forces are fighting Canadian Forces, to leave Canada without Crown consent. The section includes an exception where the accused establishes the specified assistance was not intended.",
                    "Knowing someone is about to commit treason or high treason, failing promptly to inform a justice or peace officer or make other reasonable efforts to prevent it.",
                ],
                context: "Read both alternatives and the exception in the full section.",
                penalty_section: "50", minimum: "Section 50 sets no minimum.",
                penalties: &["Indictable offence: up to 14 years' imprisonment."],
                penalty_note: "The section does not specify a fine amount.", law_sections: &["50"],
            }),
            (Category::PublicOrder, "51") => Some(ChargeGuide {
                introduction: "This offence requires both:",
                points: &["An act of violence.", "A purpose of intimidating Parliament or a provincial legislature."],
                context: "",
                penalty_section: "51", minimum: "Section 51 sets no minimum.",
                penalties: &["Indictable offence: up to 14 years' imprisonment."],
                penalty_note: "The section does not specify a fine amount.", law_sections: &["51"],
            }),
            (Category::PublicOrder, "52") => Some(ChargeGuide {
                introduction: "Sabotage concerns a prohibited act done with an intent to endanger Canada's safety, security or defence, or the safety or security of specified foreign forces lawfully in Canada.",
                points: &[
                    "A prohibited act includes an act or omission that impairs or impedes the working of a vessel, vehicle, aircraft, machine or other thing, or causes property to be lost, damaged or destroyed.",
                    "The required intent to endanger must also be considered.",
                ],
                context: "The section protects specified work stoppages and approaching a place only to obtain or communicate information. It also addresses advocacy, protest and dissent without intent to cause the specified harms. Section 52.3 requires the Attorney General's consent to institute proceedings.",
                penalty_section: "52", minimum: "Section 52 sets no minimum.",
                penalties: &["Up to 10 years on indictment, or summary conviction.", "The general summary rule in section 787 is a fine up to $5,000, imprisonment up to two years less a day, or both, unless otherwise provided by law."],
                penalty_note: "Applicable procedure, exceptions and other sentencing provisions matter.", law_sections: &["52", "52.3", "787"],
            }),
            (Category::PublicOrder, "52.1") => Some(ChargeGuide {
                introduction: "This offence concerns interfering with access to essential infrastructure or causing it to be lost, inoperable, unsafe or unfit for use, with a specified intent.",
                points: &[
                    "The intent must be to endanger Canada's safety, security or defence; endanger the safety or security of specified foreign forces lawfully in Canada; or cause a serious risk to public health or safety, including a segment of the public.",
                    "Essential infrastructure includes qualifying public or private systems such as transportation, communications, water, energy, health, food, government operations and finance.",
                ],
                context: "The section addresses specified work stoppages, obtaining or communicating information, and advocacy, protest or dissent. The Attorney General's consent is required to institute proceedings under section 52.3.",
                penalty_section: "52.1", minimum: "Section 52.1 sets no minimum.",
                penalties: &["Up to 10 years on indictment, or summary conviction.", "The general summary rule in section 787 is a fine up to $5,000, imprisonment up to two years less a day, or both, unless otherwise provided by law."],
                penalty_note: "The exact definition, intent and exceptions matter.", law_sections: &["52.1", "52.3", "787"],
            }),
            (Category::PublicOrder, "52.2") => Some(ChargeGuide {
                introduction: "This offence concerns making, possessing, selling or distributing a device, including a computer program, for use in sabotage.",
                points: &["The person must intend that the device be used, or know that it will be used, in whole or in part to carry out an offence under section 52(1) or 52.1(1)."],
                context: "The Attorney General's consent is required to institute proceedings under section 52.3.",
                penalty_section: "52.2", minimum: "Section 52.2 sets no minimum.",
                penalties: &["Up to 10 years on indictment, or summary conviction.", "The general summary rule in section 787 is a fine up to $5,000, imprisonment up to two years less a day, or both, unless otherwise provided by law."],
                penalty_note: "", law_sections: &["52.2", "52.3", "787"],
            }),
            (Category::PublicOrder, "53") => Some(ChargeGuide {
                introduction: "Inciting to mutiny includes either of these attempts involving a member of the Canadian Forces:",
                points: &["For a traitorous or mutinous purpose, attempting to draw the member away from duty and allegiance to the Sovereign.", "Attempting to incite or induce the member to commit a traitorous or mutinous act."],
                context: "",
                penalty_section: "53", minimum: "Section 53 sets no minimum.",
                penalties: &["Indictable offence: up to 14 years' imprisonment."],
                penalty_note: "The section does not specify a fine amount.", law_sections: &["53"],
            }),
            (Category::PublicOrder, "54") => Some(ChargeGuide {
                introduction: "This offence concerns aiding, assisting, harbouring or concealing a person known to be a Canadian Forces deserter or absent without leave.",
                points: &["The person must know of the deserter or absentee status."],
                context: "Proceedings cannot be instituted without the consent of the Attorney General of Canada.",
                penalty_section: "54", minimum: "Section 54 sets no minimum.",
                penalties: &["Summary conviction: generally a fine up to $5,000, imprisonment up to two years less a day, or both, under section 787 unless otherwise provided by law."],
                penalty_note: "Section 737 sets a victim surcharge, including a general $100 amount when no fine is imposed for a summary offence; the court may reduce or waive it in the circumstances stated there.",
                law_sections: &["54", "787", "737"],
            }),
            (Category::PublicOrder, "56") => Some(ChargeGuide {
                introduction: "This offence concerns wilful conduct involving a member of the Royal Canadian Mounted Police:",
                points: &["Persuading or counselling a member to desert or be absent without leave.", "Aiding, assisting, harbouring or concealing a member known to be a deserter or absent without leave.", "Helping a member desert or be absent without leave, knowing the member is about to do so."],
                context: "",
                penalty_section: "56", minimum: "Section 56 sets no minimum.",
                penalties: &["Summary conviction: generally a fine up to $5,000, imprisonment up to two years less a day, or both, under section 787 unless otherwise provided by law."],
                penalty_note: "Section 737 sets a victim surcharge, including a general $100 amount when no fine is imposed for a summary offence; the court may reduce or waive it in the circumstances stated there.",
                law_sections: &["56", "787", "737"],
            }),
            (Category::Property, "322") => Some(ChargeGuide {
                introduction: "Theft involves fraudulently taking something or converting it to your own use or someone else's, without colour of right.",
                points: &[
                    "An honest belief in a legal right may matter.",
                    "The required intent can be to deprive an owner or a person with a special interest, even temporarily; pledge it or use it as security; part with it under a return condition you may be unable to meet; or deal with it so it cannot be restored in its original condition.",
                ],
                context: "With intent to steal, moving the thing, causing it to move, or beginning to make it movable can complete the theft. A taking can be fraudulent even if done openly. Ownership is not limited to the simple situation where someone else owns the item.",
                penalty_section: "334",
                minimum: "Section 334 sets no minimum. Other provisions can apply to particular thefts.",
                penalties: &[
                    "A will or property worth more than $5,000: up to 10 years on indictment, or summary conviction.",
                    "Property worth $5,000 or less: up to 2 years on indictment, or summary conviction.",
                ],
                penalty_note: "Section 334 applies except where another law provides otherwise. Summary-conviction penalties are governed by other provisions.",
                law_sections: &["322", "334"],
            }),
            (Category::Violent, "266") => Some(ChargeGuide {
                introduction: "Section 265 defines assault. Any one of these forms can apply:",
                points: &[
                    "Force: intentionally applying force to someone, directly or indirectly, without consent.",
                    "Attempt or threat: attempting or threatening force by an act or gesture, with present ability to carry it out or causing the other person reasonably to believe that ability exists.",
                    "Weapon: while openly wearing or carrying a weapon or imitation, accosting or impeding someone, or begging.",
                ],
                context: "Section 265 says consent is not obtained where a complainant submits or does not resist because of force, threats or fear of force, fraud, or the exercise of authority. It applies to all forms of assault, including sexual assault.",
                penalty_section: "266",
                minimum: "Section 266 sets no minimum.",
                penalties: &["Common assault: up to 5 years on indictment, or summary conviction."],
                penalty_note: "Other assault offences have separate penalties; summary-conviction penalties are governed by other provisions.",
                law_sections: &["265", "266"],
            }),
            (Category::Violent, "264.1") => Some(ChargeGuide {
                introduction: "This offence involves knowingly making, conveying, or causing someone to receive a threat in any manner.",
                points: &[
                    "Threatening to cause death or bodily harm to a person.",
                    "Threatening to burn, destroy, or damage property.",
                    "Threatening to kill, poison, or injure an animal or bird that belongs to someone.",
                ],
                context: "",
                penalty_section: "264.1",
                minimum: "Section 264.1 specifies no minimum punishment.",
                penalties: &[
                    "Death or bodily-harm threat: up to 5 years on indictment, or summary conviction.",
                    "Property or animal threat: up to 2 years on indictment, or summary conviction.",
                ],
                penalty_note: "Unless another law provides otherwise, the general summary-conviction maximum is a $5,000 fine, two years less a day in prison, or both (section 787).",
                law_sections: &["264.1", "787"],
            }),
            (Category::Driving, "320.14") => Some(ChargeGuide {
                introduction: "This section covers impaired operation and specified alcohol or drug concentrations after operating a conveyance.",
                points: &[
                    "Operating while the ability to operate is impaired to any degree by alcohol, a drug, or both.",
                    "Within two hours after operation, having at least 80 mg of alcohol per 100 mL of blood, a prescribed drug concentration, or a prescribed combination of alcohol and a drug.",
                    "A separate offence covers certain lower prescribed drug concentrations within two hours after operation.",
                    "Separate offences apply where a subsection (1) offence is committed and operation causes bodily harm or death.",
                ],
                context: "The concentration offences have specific exceptions for consumption after operation. Each exception has conditions, including the person's reasonable expectation of a sample demand.",
                penalty_section: "320.19",
                minimum: "For the subsection (1) offence, the usual minimum is a $1,000 fine for a first offence, 30 days in prison for a second, and 120 days for later offences. For a first offence under paragraph (1)(b), the minimum is $1,500 at 120 to 159 mg of alcohol per 100 mL of blood and $2,000 at 160 mg or more. Sections 320.2 and 320.21 set minimums for bodily harm and death offences. Section 320.19(2) specifies no minimum for the lower-drug-concentration offence.",
                penalties: &[
                    "Subsection (1): up to 10 years on indictment, or up to two years less a day and/or a $5,000 fine on summary conviction.",
                    "Lower drug concentration under subsection (4): summary conviction, with a fine of up to $1,000.",
                    "Causing bodily harm under subsection (2): up to 14 years on indictment, or up to two years less a day and/or a $5,000 fine on summary conviction (section 320.2).",
                    "Causing death under subsection (3): indictable offence, up to life imprisonment (section 320.21).",
                ],
                penalty_note: "The applicable subsection, prior offences, other sentencing law, and any court rulings on minimums matter.",
                law_sections: &["320.14", "320.19", "320.2", "320.21"],
            }),
            (Category::Property, "348") => Some(ChargeGuide {
                introduction: "Breaking and entering can arise in any of these situations:",
                points: &[
                    "Breaking and entering a place intending to commit an indictable offence inside.",
                    "Breaking and entering a place and committing an indictable offence inside.",
                    "Breaking out of a place after committing an indictable offence there, or after entering with intent to commit one.",
                ],
                context: "A place includes a dwelling, other building or structure or part of one, railway vehicle, vessel, aircraft, trailer, or certain pens for fur-bearing animals. Section 348(2) sets out presumptions that apply in the absence of evidence to the contrary; their application depends on the evidence and law.",
                penalty_section: "348",
                minimum: "Section 348 sets no minimum.",
                penalties: &[
                    "Dwelling-house: indictable offence, up to life imprisonment.",
                    "Other place: up to 10 years on indictment, or summary conviction.",
                ],
                penalty_note: "",
                law_sections: &["348"],
            }),
            (Category::Property, "380") => Some(ChargeGuide {
                introduction: "Fraud under section 380(1) concerns depriving the public or a person of property, money, valuable security or a service through deceit, falsehood or other fraudulent means.",
                points: &[
                    "What dishonest act and deprivation or risk of deprivation are alleged?",
                    "The person affected can be identified or unidentified; the public can also be affected.",
                ],
                context: "Section 380(2) also addresses fraudulently affecting a public market price and has its own wording.",
                penalty_section: "380",
                minimum: "The section specifies a 2-year minimum when prosecuted by indictment and the total value of the relevant fraud convictions exceeds $1 million; applicability can depend on other law.",
                penalties: &[
                    "A will or subject matter worth more than $5,000: indictable offence, up to 14 years.",
                    "Subject matter worth $5,000 or less: up to 2 years on indictment, or summary conviction.",
                ],
                penalty_note: "For an individual, section 734 generally permits a fine in addition to a required minimum prison term, not in place of it. Rules for organizations and restitution differ; the statute and applicable case law govern.",
                law_sections: &["380"],
            }),
            (Category::Property, "430") => Some(ChargeGuide {
                introduction: "Mischief to property includes wilfully doing any of the following:",
                points: &[
                    "Destroying or damaging property.",
                    "Making property dangerous, useless, inoperative or ineffective.",
                    "Obstructing, interrupting or interfering with the lawful use, enjoyment or operation of property, or with a person lawfully using it.",
                ],
                context: "A separate form covers computer data. Stopping work in the circumstances described in section 430(6), or approaching a place only to obtain or communicate information, is not by itself mischief.",
                penalty_section: "430",
                minimum: "For specified war memorial property, the section sets minimums of a $1,000 fine for a first offence, 14 days' imprisonment for a second, and 30 days for later offences. Applicability can depend on other law.",
                penalties: &[
                    "Actual danger to life: indictable offence, up to life imprisonment.",
                    "A will or property worth more than $5,000: up to 10 years on indictment, or summary conviction; other property: up to 2 years on indictment, or summary conviction.",
                    "Specified religious, educational, community or seniors' property, with the bias, prejudice or hate described in the section: up to 10 years on indictment, or summary conviction.",
                    "Specified war memorial property: up to 10 years on indictment, or two years less a day on summary conviction.",
                    "Computer data or specified cultural property: up to 10 years on indictment, or summary conviction.",
                ],
                penalty_note: "The applicable subsection depends on the facts. The full section includes further offences and limits.",
                law_sections: &["430"],
            }),
            _ => additional_guides::lookup(self),
        }
    }

    pub fn breakdown(&self) -> Option<Breakdown> {
        match (self.category, self.number) {
            (Category::Drugs, "4(1)") => Some(Breakdown {
                overview: "This provision concerns possessing a substance listed in Schedule I, II or III without authorization under the regulations.",
                checks: &[
                    "What substance is alleged, and is it listed in one of those schedules?",
                    "What evidence supports possession? The legal meaning of possession is not limited to having something in a pocket.",
                    "Was possession authorized? Could the medical-emergency protections in section 4.1 apply?",
                ],
                limits: "The substance, authorization, applicable exemptions and evidence matter. Section 4.1 has specific conditions; it does not automatically apply in every emergency. Penalties differ by schedule and procedure.",
            }),
            (Category::Drugs, "5(1)") => Some(Breakdown {
                overview: "This provision concerns trafficking in substances listed in Schedules I to V, or substances the person represents as being listed there.",
                checks: &[
                    "Which substance or representation is alleged, and which schedule is involved?",
                    "What conduct is alleged? The definition of traffic in section 2 includes more than selling, such as giving, transferring and certain offers.",
                    "Do an authorization or other applicable exceptions affect the alleged conduct?",
                ],
                limits: "The legal definition and its exceptions matter. Punishment depends on the substance and applicable procedure. Read sections 2 and 5 together.",
            }),
            (Category::Drugs, "5(2)") => Some(Breakdown {
                overview: "This provision concerns possessing a substance listed in Schedules I to V for the purpose of trafficking.",
                checks: &[
                    "What substance is alleged, and is it listed in one of those schedules?",
                    "What evidence supports possession, and what evidence supports a purpose of trafficking?",
                    "What does the definition of traffic in section 2 cover, and do its exceptions or other applicable rules matter?",
                ],
                limits: "Possession alone does not establish the stated purpose. The full facts, legal definitions, applicable exceptions and substance schedule matter.",
            }),
            _ => None,
        }
    }

    pub fn act_code(&self) -> &'static str {
        if self.category == Category::Drugs { "C-38.8" } else { "C-46" }
    }

    pub fn act_name(&self) -> &'static str {
        if self.category == Category::Drugs { "Controlled Drugs and Substances Act" } else { "Criminal Code" }
    }

    pub fn statute_url(&self) -> String {
        let act = self.act_code();
        let section = self.number.split('(').next().unwrap_or(self.number);
        format!("https://laws-lois.justice.gc.ca/eng/acts/{act}/section-{section}.html")
    }

    pub fn related_section(&self) -> Option<(&'static str, &'static str)> {
        match (self.category, self.number) {
            (Category::Violent, "236") => Some(("Definition", "234")),
            (Category::Violent, "266") => Some(("Definition", "265")),
            (Category::Violent, "344") => Some(("Definition", "343")),
            (Category::PublicOrder, "132") => Some(("Definition", "131")),
            (Category::Property, "322") => Some(("Punishment", "334")),
            (Category::Property, "354") => Some(("Punishment", "355")),
            (Category::Property, "366") => Some(("Punishment", "367")),
            (Category::Violent, "229") => Some(("Punishment", "235")),
            (Category::Driving, "320.14" | "320.15") => Some(("Related punishment", "320.19")),
            (Category::Drugs, "4(1)") => Some(("Medical emergency protections", "4.1")),
            (Category::Drugs, "5(1)" | "5(2)") => Some(("Definitions", "2")),
            _ => None,
        }
    }
}

const fn c(
    number: &'static str,
    name: &'static str,
    category: Category,
    kind: Kind,
    penalty: &'static str,
    plain: &'static str,
) -> Charge {
    Charge { number, name, category, kind, penalty, plain }
}

use Category::*;
use Kind::*;

pub static CHARGES: &[Charge] = &[
    // --- Violent -----------------------------------------------------------------
    c("220", "Causing death by criminal negligence", Violent, Indictable, "Life", "Fail to take care where death results."),
    c("229", "Murder", Violent, Indictable, "Life", "Culpable homicide that meets the criteria for murder."),
    c("236", "Manslaughter", Violent, Indictable, "Life", "Culpable homicide that is neither murder nor infanticide."),
    c("264", "Criminal harassment", Violent, Hybrid, "10 years", "Repeated conduct that makes someone fear."),
    c("264.1", "Uttering threats", Violent, Hybrid, "5 years", "Threat to cause death or harm."),
    c("266", "Assault", Violent, Hybrid, "5 years", "The Criminal Code defines several forms of assault."),
    c("267", "Assault with a weapon or causing bodily harm", Violent, Hybrid, "10 years", "Assault that injures, or uses a weapon."),
    c("268", "Aggravated assault", Violent, Indictable, "14 years", "Assault that wounds, maims or endangers life."),
    c("271", "Sexual assault", Violent, Hybrid, "10 years", "Sexual contact without consent."),
    c("279", "Kidnapping", Violent, Indictable, "Life", "Kidnapping with intent to confine, remove from Canada, hold for ransom or force into service against the person's will."),
    c("344", "Robbery", Violent, Indictable, "Life", "Theft with violence, threats or an offensive weapon, or assault with intent to steal."),
    // --- Property ----------------------------------------------------------------
    c("322", "Theft", Property, Hybrid, "10 years", "Fraudulently taking or converting property without colour of right, with an intent specified in the Code."),
    c("333.1", "Theft of a motor vehicle", Property, Hybrid, "10 years", "Taking a vehicle without consent."),
    c("348", "Breaking and entering", Property, Varies, "Life", "Breaking and entering a place with intent to commit an indictable offence, or committing one there."),
    c("351", "Possession of break-in instruments", Property, Hybrid, "10 years", "Possession without lawful excuse of a break-in instrument, knowing of its use or intended use for that purpose."),
    c("354", "Possession of property obtained by crime", Property, Hybrid, "10 years", "Possession of property or proceeds knowing they were obtained by certain crimes."),
    c("366", "Forgery", Property, Hybrid, "10 years", "Knowingly making a false document with intent that it be used as genuine or induce someone to act on it."),
    c("380", "Fraud", Property, Hybrid, "14 years", "Obtaining property or money by deceit."),
    c("430", "Mischief", Property, Hybrid, "10 years", "Damaging or destroying property."),
    c("434", "Arson — damage to property", Property, Indictable, "14 years", "Intentionally or recklessly damaging property by fire or explosion."),
    c("462.31", "Laundering proceeds of crime", Property, Hybrid, "10 years", "Cleaning money obtained illegally."),
    // --- Weapons -----------------------------------------------------------------
    c("89", "Carrying a weapon at a public meeting", Weapons, Summary, "Varies", "Carrying a specified item at or on the way to a public meeting without lawful excuse."),
    c("92(1)", "Possessing a firearm knowing it is unauthorized", Weapons, Indictable, "10 years", "Possessing a firearm knowing of the specified licensing or registration deficiency."),
    c("92(2)", "Possessing a prohibited weapon or device knowingly", Weapons, Indictable, "10 years", "Possessing a specified weapon, device or ammunition knowing of the licensing deficiency."),
    c("93", "Possession at an unauthorized place", Weapons, Hybrid, "5 years", "Possessing a specified item at a place not authorized by the licence or law."),
    c("94", "Unauthorized possession in a motor vehicle", Weapons, Hybrid, "10 years", "Being an occupant of a vehicle knowing a specified item is present in the circumstances described by law."),
    c("96", "Possession of a weapon obtained by offence", Weapons, Hybrid, "14 years", "Possessing a specified item knowing it was obtained by an offence."),
    c("98", "Breaking and entering to steal a firearm", Weapons, Indictable, "Life", "Breaking and entering or breaking out in the circumstances described by section 98."),
    c("98.1", "Robbery to steal a firearm", Weapons, Indictable, "Life", "Committing robbery with intent to steal a firearm or stealing one during the robbery."),
    c("99", "Weapons trafficking", Weapons, Indictable, "14 years", "Manufacturing or transferring specified items knowing the act is unauthorized."),
    c("108", "Tampering with a firearm serial number", Weapons, Hybrid, "5 years", "Altering or removing a serial number, or knowingly possessing a firearm with a tampered number, without lawful excuse."),
    c("100", "Possession for weapons trafficking", Weapons, Indictable, "14 years", "Possessing specified items for unauthorized transfer, knowing the transfer is unauthorized."),
    c("101", "Transfer without authority", Weapons, Hybrid, "5 years", "Transferring specified weapons or related items without authorization."),
    c("102", "Making an automatic firearm", Weapons, Hybrid, "10 years", "Making, assembling or altering a firearm for automatic fire without lawful excuse."),
    c("102.1", "Computer data for making firearms", Weapons, Hybrid, "10 years", "Possessing or distributing specified computer data for making firearms or prohibited devices."),
    c("103", "Unauthorized importing or exporting with knowledge", Weapons, Indictable, "14 years", "Importing or exporting specified items knowing it is unauthorized."),
    c("104", "Unauthorized importing or exporting", Weapons, Hybrid, "5 years", "Importing or exporting specified items without authorization."),
    c("104.1", "Altering a cartridge magazine", Weapons, Hybrid, "5 years", "Altering a magazine into a prohibited device without lawful excuse."),
    c("105", "Failing to report a lost or found weapon", Weapons, Hybrid, "5 years", "Failing to report specified lost, stolen or found items as required."),
    c("106", "Failing to report a destroyed weapon", Weapons, Hybrid, "5 years", "Failing to report destruction of a specified firearm, weapon, device or ammunition."),
    c("107", "False reports about weapons", Weapons, Hybrid, "5 years", "Knowingly making a false report about the loss, theft or destruction of specified items."),
    c("85", "Use of a firearm in an offence", Weapons, Indictable, "14 years", "Using a firearm while committing an offence."),
    c("86", "Careless use of a firearm", Weapons, Hybrid, "5 years", "Storing or using a firearm without care."),
    c("87", "Pointing a firearm", Weapons, Hybrid, "5 years", "Aiming a firearm at another person."),
    c("88", "Weapon for a dangerous purpose", Weapons, Hybrid, "10 years", "Carrying a weapon to do harm."),
    c("90", "Carrying a concealed weapon", Weapons, Hybrid, "5 years", "A weapon hidden on the person."),
    c("91", "Unauthorized possession of a firearm", Weapons, Hybrid, "5 years", "Holding a firearm without a licence."),
    c("95", "Prohibited or restricted firearm with ammunition", Weapons, Hybrid, "14 years", "Possessing a prohibited or restricted firearm with ammunition without the required authorization."),
    c("117.01", "Possession contrary to an order", Weapons, Hybrid, "10 years", "Holding a weapon despite a prohibition."),
    // --- Drugs -------------------------------------------------------------------
    c("4(1)", "Possession of a controlled substance", Drugs, Varies, "Varies", "Possession of specified controlled substances without authorization."),
    c("5(1)", "Trafficking", Drugs, Varies, "Varies", "Trafficking in a controlled substance."),
    c("5(2)", "Possession for the purpose of trafficking", Drugs, Varies, "Varies", "Possession of a controlled substance for the purpose of trafficking."),
    c("6(1)", "Importing a controlled substance", Drugs, Varies, "Varies", "Importing a controlled substance without authorization."),
    c("7(1)", "Production of a controlled substance", Drugs, Varies, "Varies", "Producing a controlled substance without authorization."),
    // --- Driving -----------------------------------------------------------------
    c("320.13", "Dangerous operation of a conveyance", Driving, Hybrid, "10 years", "Driving in a way that endangers the public."),
    c("320.14", "Operation while impaired", Driving, Hybrid, "10 years", "Operating while impaired by alcohol or drugs, or exceeding a prescribed blood concentration."),
    c("320.15", "Refusing a breath or blood sample", Driving, Hybrid, "10 years", "Declining a lawful screening demand."),
    c("320.17", "Flight from a peace officer", Driving, Hybrid, "10 years", "Failing to stop when signalled."),
    c("320.18", "Operation while prohibited", Driving, Hybrid, "10 years", "Driving while disqualified."),
    // --- Public order ------------------------------------------------------------
    c("83.21", "Instructing activity for a terrorist group", PublicOrder, Indictable, "Life", "Knowingly instructing specified activity for a terrorist group for the purpose stated in the Code."),
    c("83.22", "Instructing terrorist activity", PublicOrder, Indictable, "Life", "Knowingly instructing a person to carry out a terrorist activity."),
    c("83.221", "Counselling a terrorism offence", PublicOrder, Indictable, "5 years", "Counselling another person to commit a terrorism offence without identifying a specific offence."),
    c("83.23(1)", "Concealing a person who carried out terrorist activity", PublicOrder, Indictable, "Varies", "Knowingly harbouring or concealing a person who carried out terrorist activity for the specified purpose."),
    c("83.23(2)", "Concealing a person likely to carry out terrorist activity", PublicOrder, Indictable, "10 years", "Knowingly harbouring or concealing a person likely to carry out terrorist activity for the specified purpose."),
    c("83.231", "Hoax involving terrorist activity", PublicOrder, Hybrid, "Varies", "A specified hoax involving a reasonable apprehension of terrorist activity."),
    c("119", "Bribery of judicial officers and legislators", PublicOrder, Indictable, "14 years", "Corruptly offering or accepting specified benefits involving judicial officers or legislators."),
    c("120", "Bribery of officers", PublicOrder, Indictable, "14 years", "Corruptly offering or accepting specified benefits involving justice officials."),
    c("121(1)", "Frauds on the government", PublicOrder, Hybrid, "5 years", "Specified transactions involving government officials, benefits, influence or contracts."),
    c("121(2)", "Contractor subscribing to an election fund", PublicOrder, Hybrid, "5 years", "Providing specified benefits connected to a government contract to influence an election."),
    c("121.1", "Unstamped tobacco products", PublicOrder, Hybrid, "5 years", "Specified dealings in tobacco products or raw leaf tobacco contrary to the section."),
    c("122", "Breach of trust by public officer", PublicOrder, Hybrid, "5 years", "Fraud or breach of trust by an official in connection with their duties."),
    c("123(1)", "Municipal corruption", PublicOrder, Hybrid, "5 years", "Specified benefits provided or accepted for a municipal official's actions."),
    c("123(2)", "Influencing a municipal official", PublicOrder, Hybrid, "5 years", "Influencing a municipal official by specified unlawful means."),
    c("124", "Selling or purchasing office", PublicOrder, Hybrid, "5 years", "Specified purported sales or purchases involving appointment to or resignation from office."),
    c("125", "Influencing or negotiating appointments", PublicOrder, Hybrid, "5 years", "Specified benefits, negotiations or unauthorized dealings involving public office."),
    c("141", "Compounding an indictable offence", PublicOrder, Hybrid, "2 years", "Accepting or seeking valuable consideration for agreeing to compound or conceal an indictable offence."),
    c("142", "Corruptly taking a reward for recovery of goods", PublicOrder, Hybrid, "5 years", "Corruptly accepting valuable consideration for purported help recovering property obtained by an indictable offence."),
    c("126", "Disobeying a federal statute", PublicOrder, Hybrid, "2 years", "Intentionally breaking a federal Act without lawful excuse where no punishment is expressly provided."),
    c("128", "Misconduct by an officer executing process", PublicOrder, Hybrid, "2 years", "Specified intentional misconduct or a false return by a peace officer or coroner entrusted with a process."),
    c("134", "False sworn statement without legal authority", PublicOrder, Summary, "Varies", "Knowingly making a false sworn statement when not specially permitted, authorized or required by law."),
    c("136", "Witness giving contradictory evidence", PublicOrder, Indictable, "14 years", "Giving contradictory material evidence in judicial proceedings with intent to mislead."),
    c("137", "Fabricating evidence", PublicOrder, Indictable, "14 years", "Fabricating material for use as evidence in a judicial proceeding with intent to mislead."),
    c("138", "Offences relating to affidavits", PublicOrder, Hybrid, "2 years", "Specified misconduct involving affidavits or statutory declarations."),
    c("811", "Breach of recognizance", PublicOrder, Hybrid, "4 years", "Breaching a recognizance under one of the listed peace-bond provisions."),
    c("810.4", "Misuse of peace-bond samples or test results", PublicOrder, Summary, "Varies", "Misusing bodily samples or test results collected under listed recognizances."),
    c("83.03(1)", "Providing property for terrorist activity", PublicOrder, Indictable, "10 years", "Providing or collecting property or services for specified terrorist purposes."),
    c("83.03(2)", "Providing property for a terrorist group", PublicOrder, Indictable, "10 years", "Providing or collecting property or services knowing they will be used by or benefit a terrorist group."),
    c("83.04", "Using or possessing property for terrorist purposes", PublicOrder, Indictable, "10 years", "Using or possessing property for specified terrorist activity."),
    c("83.12", "Contravening terrorist property duties", PublicOrder, Hybrid, "10 years", "Contravening specified property, disclosure or audit provisions."),
    c("83.18", "Participating in terrorist group activity", PublicOrder, Indictable, "10 years", "Knowingly participating in or contributing to a terrorist group activity for a specified purpose."),
    c("83.181", "Leaving Canada to participate in terrorist group activity", PublicOrder, Indictable, "10 years", "Leaving or attempting to leave Canada for specified participation abroad."),
    c("83.19", "Facilitating terrorist activity", PublicOrder, Indictable, "14 years", "Knowingly facilitating a terrorist activity."),
    c("83.191", "Leaving Canada to facilitate terrorist activity", PublicOrder, Indictable, "14 years", "Leaving or attempting to leave Canada for specified facilitation abroad."),
    c("83.2", "Committing an offence for a terrorist group", PublicOrder, Indictable, "Life", "Committing an indictable offence for or in association with a terrorist group."),
    c("83.201", "Leaving Canada to commit an offence for a terrorist group", PublicOrder, Indictable, "14 years", "Leaving or attempting to leave Canada for a specified offence abroad involving a terrorist group."),
    c("83.202", "Leaving Canada to commit a terrorist offence", PublicOrder, Indictable, "14 years", "Leaving or attempting to leave Canada for an offence abroad that is terrorist activity."),
    c("56.1", "Identity documents", PublicOrder, Hybrid, "5 years", "Possessing or dealing with another person's identity document without lawful excuse."),
    c("57(1)", "Forgery or use of a forged passport", PublicOrder, Indictable, "14 years", "Forging or knowingly dealing with a forged passport."),
    c("57(2)", "False statement to obtain a passport", PublicOrder, Hybrid, "2 years", "Knowingly making a false or misleading statement to obtain or alter a passport."),
    c("57(3)", "Possession of a forged passport", PublicOrder, Hybrid, "5 years", "Possessing a forged passport or one obtained through a false statement without lawful excuse."),
    c("58", "Fraudulent use of citizenship certificate", PublicOrder, Hybrid, "2 years", "Using or handing over a certificate for a fraudulent purpose."),
    c("61", "Seditious offences", PublicOrder, Indictable, "14 years", "Seditious words, libel or conspiracy."),
    c("62", "Offences involving military forces", PublicOrder, Hybrid, "5 years", "Specified intentional interference with the loyalty or discipline of force members."),
    c("65", "Taking part in a riot", PublicOrder, Hybrid, "Varies", "Participating in a riot, with separate provisions for disguise."),
    c("66", "Unlawful assembly", PublicOrder, Varies, "Varies", "Membership in an unlawful assembly, with separate provisions for disguise."),
    c("68", "Offences related to proclamation", PublicOrder, Indictable, "Life", "Specified conduct involving a proclamation to disperse a riot."),
    c("69", "Neglect by peace officer", PublicOrder, Hybrid, "2 years", "Failure without reasonable excuse to take reasonable steps to suppress a riot."),
    c("70(3)", "Contravening an unlawful drilling order", PublicOrder, Hybrid, "5 years", "Contravening an order made under section 70."),
    c("72(1)", "Forcible entry", PublicOrder, Hybrid, "2 years", "Entering real property in a manner likely to cause a breach of the peace."),
    c("72(2)", "Forcible detainer", PublicOrder, Hybrid, "2 years", "Detaining real property in a manner likely to cause a breach of the peace."),
    c("74", "Piracy by law of nations", PublicOrder, Indictable, "Life", "An act of piracy under the law of nations."),
    c("75", "Piratical acts", PublicOrder, Indictable, "14 years", "Specified theft, damage, mutiny or counselling involving a Canadian ship."),
    c("76", "Hijacking", PublicOrder, Indictable, "Life", "Unlawfully seizing or controlling an aircraft by force or intimidation with specified intent."),
    c("77", "Endangering aircraft or airport safety", PublicOrder, Indictable, "Life", "Specified acts endangering aircraft or airport safety."),
    c("78", "Weapons and explosives on civil aircraft", PublicOrder, Indictable, "14 years", "Taking an offensive weapon or explosive onto a civil aircraft in specified circumstances."),
    c("78.1", "Offences against ships and fixed platforms", PublicOrder, Indictable, "Varies", "Specified seizure, violence, damage, threats or false information involving ships or platforms."),
    c("80", "Breach of duty concerning explosives", PublicOrder, Indictable, "Life", "Failure to perform a legal duty concerning explosives where an explosion results."),
    c("81", "Using explosives", PublicOrder, Indictable, "Varies", "Specified use or possession of an explosive with prohibited intent."),
    c("82(1)", "Possession of an explosive", PublicOrder, Hybrid, "5 years", "Making or possessing an explosive without lawful excuse."),
    c("82(2)", "Explosive associated with a criminal organization", PublicOrder, Indictable, "14 years", "Making or possessing an explosive without lawful excuse in association with a criminal organization."),
    c("82.3", "Nuclear or radioactive material with intent to harm", PublicOrder, Indictable, "Life", "Specified conduct involving nuclear or radioactive material with intent to cause serious harm."),
    c("82.4", "Nuclear or radioactive material used to compel", PublicOrder, Indictable, "Life", "Specified conduct involving nuclear or radioactive material with intent to compel."),
    c("82.5", "Offence to obtain nuclear material", PublicOrder, Indictable, "Life", "Committing an indictable offence with intent to obtain nuclear material or access."),
    c("82.6", "Threats involving nuclear material", PublicOrder, Indictable, "14 years", "Threatening to commit specified nuclear material offences."),
    c("83", "Engaging in a prize fight", PublicOrder, Summary, "Varies", "Specified involvement in a prize fight, subject to statutory exclusions."),
    c("83.02", "Providing property for terrorist activities", PublicOrder, Indictable, "10 years", "Wilfully providing or collecting property for specified terrorist acts."),
    c("46(1)", "High treason", PublicOrder, Indictable, "Life", "Specified acts against the Sovereign, Canada or Canadian Forces."),
    c("46(2)", "Treason", PublicOrder, Indictable, "Varies", "Specified force, information, conspiracy or intention offences."),
    c("50", "Assisting an enemy to leave or omitting to prevent treason", PublicOrder, Indictable, "14 years", "Specified assistance or failure to prevent treason."),
    c("51", "Intimidating Parliament or a legislature", PublicOrder, Indictable, "14 years", "An act of violence to intimidate Parliament or a provincial legislature."),
    c("52", "Sabotage", PublicOrder, Hybrid, "10 years", "A prohibited act with a specified intent to endanger security or defence."),
    c("52.1", "Sabotage — essential infrastructure", PublicOrder, Hybrid, "10 years", "Interference with essential infrastructure with a specified intent."),
    c("52.2", "Sabotage — device", PublicOrder, Hybrid, "10 years", "A device made, possessed, sold or distributed for specified sabotage."),
    c("53", "Inciting to mutiny", PublicOrder, Indictable, "14 years", "Specified attempts involving members of the Canadian Forces."),
    c("54", "Assisting a deserter", PublicOrder, Summary, "Varies", "Aiding or concealing a known deserter or absentee from the Canadian Forces."),
    c("56", "Offences involving RCMP members", PublicOrder, Summary, "Varies", "Specified conduct involving RCMP desertion or absence without leave."),
    c("127", "Disobeying a court order", PublicOrder, Hybrid, "2 years", "Breaking a court's order."),
    c("129", "Offences involving public or peace officers", PublicOrder, Hybrid, "2 years", "Specified obstruction, resistance or failure to assist an officer."),
    c("130", "Personating a peace or public officer", PublicOrder, Hybrid, "5 years", "Falsely representing that one is an officer or using specified items in a misleading way."),
    c("132", "Perjury", PublicOrder, Indictable, "14 years", "Knowingly making a false statement with intent to mislead in specified legal settings."),
    c("139", "Obstructing justice", PublicOrder, Hybrid, "10 years", "Interfering with the justice system."),
    c("140", "Public mischief", PublicOrder, Hybrid, "5 years", "Causing a police investigation falsely."),
    c("145", "Failure to appear or comply", PublicOrder, Hybrid, "2 years", "Ignoring a court or release condition."),
    c("175", "Causing a disturbance", PublicOrder, Summary, "2 years", "Noise or conduct disturbing the peace."),
    c("177", "Trespassing at night", PublicOrder, Summary, "Under 2 years", "Loitering or prowling near a dwelling at night without lawful excuse."),
    c("733.1", "Breach of probation", PublicOrder, Hybrid, "4 years", "Breaking a probation order."),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_obtained_by_crime_uses_the_offence_section() {
        let charge = CHARGES.iter().find(|c| c.name == "Possession of property obtained by crime").unwrap();
        assert_eq!(charge.number, "354");
        assert_eq!(charge.statute_url(), "https://laws-lois.justice.gc.ca/eng/acts/C-46/section-354.html");
    }

    #[test]
    fn displayed_law_excludes_supporting_provisions_but_keeps_offence_and_penalty() {
        for (category, number, expected) in [
            (Category::Property, "322", &["322", "334"][..]),
            (Category::Violent, "266", &["265", "266"][..]),
            (Category::PublicOrder, "132", &["131", "132"][..]),
            (Category::PublicOrder, "46(2)", &["46", "47"][..]),
            (Category::PublicOrder, "83.12", &["83.12"][..]),
            (Category::PublicOrder, "72(1)", &["72", "73"][..]),
            (Category::Drugs, "5(1)", &["5"][..]),
            (Category::Violent, "344", &["344", "343"][..]),
        ] {
            let charge = CHARGES.iter().find(|c| c.category == category && c.number == number).unwrap();
            assert_eq!(charge.displayed_law_sections(), expected, "{number}");
        }
    }

    #[test]
    fn related_sections_link_to_the_definition_or_punishment() {
        for (number, category, expected_label, expected_section) in [
            ("236", Category::Violent, "Definition", "234"),
            ("266", Category::Violent, "Definition", "265"),
            ("344", Category::Violent, "Definition", "343"),
            ("132", Category::PublicOrder, "Definition", "131"),
            ("322", Category::Property, "Punishment", "334"),
            ("354", Category::Property, "Punishment", "355"),
            ("366", Category::Property, "Punishment", "367"),
            ("229", Category::Violent, "Punishment", "235"),
            ("320.14", Category::Driving, "Related punishment", "320.19"),
            ("320.15", Category::Driving, "Related punishment", "320.19"),
            ("4(1)", Category::Drugs, "Medical emergency protections", "4.1"),
            ("5(1)", Category::Drugs, "Definitions", "2"),
            ("5(2)", Category::Drugs, "Definitions", "2"),
        ] {
            let charge = CHARGES.iter().find(|c| c.number == number && c.category == category).unwrap();
            assert_eq!(charge.related_section(), Some((expected_label, expected_section)));
        }
        assert_eq!(CHARGES.iter().find(|c| c.category == Category::Drugs && c.number == "6(1)").unwrap().related_section(), None);
    }

    #[test]
    fn example_guides_have_explanation_penalty_and_complete_source_sections() {
        for (category, number, sections) in [
            (Category::Property, "322", &["322", "334"][..]),
            (Category::Violent, "266", &["265", "266"][..]),
            (Category::Violent, "264.1", &["264.1", "787"][..]),
            (Category::Driving, "320.14", &["320.14", "320.19", "320.2", "320.21"][..]),
            (Category::Property, "348", &["348"][..]),
            (Category::Property, "380", &["380"][..]),
            (Category::Property, "430", &["430"][..]),
            (Category::PublicOrder, "46(1)", &["46", "47", "734"][..]),
            (Category::PublicOrder, "46(2)", &["46", "47", "48"][..]),
            (Category::PublicOrder, "50", &["50"][..]),
            (Category::PublicOrder, "51", &["51"][..]),
            (Category::PublicOrder, "52", &["52", "52.3", "787"][..]),
            (Category::PublicOrder, "52.1", &["52.1", "52.3", "787"][..]),
            (Category::PublicOrder, "52.2", &["52.2", "52.3", "787"][..]),
            (Category::PublicOrder, "53", &["53"][..]),
            (Category::PublicOrder, "54", &["54", "787", "737"][..]),
            (Category::PublicOrder, "56", &["56", "787", "737"][..]),
        ] {
            let charge = CHARGES.iter().find(|c| c.number == number && c.category == category).unwrap();
            let guide = charge.guide().expect("example guide is available");
            assert!(!guide.introduction.is_empty() && !guide.points.is_empty());
            assert!(!guide.minimum.is_empty() && !guide.penalties.is_empty());
            assert_eq!(guide.law_sections, sections);
            assert!(guide.law_sections.contains(&guide.penalty_section));
            for section in guide.law_sections {
                assert!(crate::statutes::lookup(charge.act_code(), section).is_some());
            }
        }
    }

    #[test]
    fn completed_threat_and_impaired_guides_distinguish_penalties() {
        let threats = CHARGES.iter().find(|c| c.category == Category::Violent && c.number == "264.1").unwrap();
        let guide = threats.guide().unwrap();
        assert_eq!(guide.penalties.len(), 2);
        assert!(guide.penalties[0].contains("5 years"));
        assert!(guide.penalties[1].contains("2 years"));
        assert!(threats.breakdown().is_none());

        let impaired = CHARGES.iter().find(|c| c.category == Category::Driving && c.number == "320.14").unwrap();
        let guide = impaired.guide().unwrap();
        assert!(guide.minimum.contains("$1,500") && guide.minimum.contains("$2,000"));
        assert!(guide.penalties.iter().any(|p| p.contains("subsection (4)") && p.contains("$1,000")));
        assert!(guide.penalties.iter().any(|p| p.contains("bodily harm") && p.contains("14 years")));
        assert!(guide.penalties.iter().any(|p| p.contains("death") && p.contains("life")));
        assert!(impaired.breakdown().is_none());
    }

    #[test]
    fn detailed_drafts_are_attached_to_the_intended_offences() {
        for (category, number) in [(Category::Drugs, "4(1)"), (Category::Drugs, "5(1)"), (Category::Drugs, "5(2)")] {
            let charge = CHARGES.iter().find(|c| c.category == category && c.number == number).unwrap();
            let draft = charge.breakdown().expect("a detailed draft is available");
            assert!(!draft.overview.is_empty());
            assert!(draft.checks.len() >= 2);
            assert!(!draft.limits.is_empty());
        }
        assert!(CHARGES.iter().find(|c| c.category == Category::Drugs && c.number == "6(1)").unwrap().breakdown().is_none());
    }

    #[test]
    fn section_links_use_the_correct_act_and_strip_subsection() {
        let assault = CHARGES.iter().find(|c| c.number == "266").unwrap();
        let trafficking = CHARGES.iter().find(|c| c.number == "5(2)").unwrap();
        assert_eq!(assault.statute_url(), "https://laws-lois.justice.gc.ca/eng/acts/C-46/section-266.html");
        assert_eq!(trafficking.statute_url(), "https://laws-lois.justice.gc.ca/eng/acts/C-38.8/section-5.html");
    }
}
