//! Draft explanations paired with official Criminal Code section reproductions.
use super::{Category, Charge, ChargeGuide};

const SUMMARY: &str = "Unless another law says otherwise, the general summary maximum is a $5,000 fine, two years less a day in prison, or both (section 787). A victim surcharge may apply; section 737 permits reduction or waiver in specified circumstances.";
const INDICTABLE: &str = "The cited provision does not set a specific fine amount.";

macro_rules! guide {
    ($intro:expr, [$($point:expr),* $(,)?], $context:expr, $section:expr, [$($penalty:expr),* $(,)?], $note:expr, [$($law:expr),* $(,)?]) => {
        Some(ChargeGuide {
            introduction: $intro, points: &[$($point),*], context: $context,
            penalty_section: $section,
            minimum: "No minimum is specified in the cited offence or punishment provision.",
            penalties: &[$($penalty),*], penalty_note: $note, law_sections: &[$($law),*],
        })
    }
}

pub(super) fn lookup(charge: &Charge) -> Option<ChargeGuide> {
    if charge.category == Category::Weapons { return weapons_guide(charge.number); }
    if charge.category != Category::PublicOrder { return None; }
    match charge.number {
        "126" => guide!(
            "This offence concerns breaking an Act of Parliament without lawful excuse.",
            ["A person intentionally does something the Act forbids, or intentionally fails to do something it requires.", "No punishment is expressly provided by law for the breach."],
            "Subsection (2) addresses when the Government of Canada may bring proceedings involving an Act other than the Criminal Code.",
            "126", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["126", "787"]),
        "127" => guide!(
            "This offence concerns disobeying a lawful order without lawful excuse.",
            ["The order was made by a court or by a person or body authorized by an Act to make it.", "The section excludes orders for payment of money and applies where no punishment or other mode of proceeding is expressly provided by law."],
            "Subsection (2) addresses when the Government of Canada may bring proceedings about an order made in proceedings it instituted and conducted.",
            "127", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["127", "787"]),
        "128" => guide!(
            "This offence applies to a peace officer or coroner entrusted with carrying out a legal process.",
            ["The person intentionally behaves improperly in carrying out the process, or intentionally makes a false return about it."],
            "",
            "128", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["128", "787"]),
        "129" => guide!(
            "This section covers three kinds of conduct involving officers or lawful enforcement.",
            ["Resisting or wilfully obstructing a public officer or peace officer carrying out a duty, or a person lawfully helping the officer.", "Without reasonable excuse, failing to assist an officer with an arrest or keeping the peace after reasonable notice that help is required.", "Resisting or wilfully obstructing a person lawfully executing a process against land or goods, or making a lawful distress or seizure."],
            "",
            "129", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["129", "787"]),
        "130" => guide!(
            "This offence concerns falsely posing as a peace officer or public officer.",
            ["Falsely representing that one is a peace officer or public officer; or", "While not an officer, using a badge, uniform item or equipment in a way likely to make people believe one is an officer."],
            "Section 130.1 directs the court to consider it an aggravating circumstance when the personation was for the purpose of facilitating another offence.",
            "130", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["130", "130.1", "787"]),
        "132" => guide!(
            "Perjury is defined in section 131 and punished under section 132.",
            ["With intent to mislead, a person knowingly makes a false statement under oath or solemn affirmation before someone authorized by law to receive it.", "The section also covers certain evidence or statements given by video link or under an international assistance order, subject to its stated conditions."],
            "The section addresses who may make the statement and whether it is made in a judicial proceeding. Section 133 requires corroboration in the circumstances it describes; section 134 separately addresses certain false sworn statements.",
            "132", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["131", "132", "133"]),
        "134" => guide!(
            "This offence concerns certain knowingly false statements made under oath or solemn affirmation by a person not specially permitted, authorized or required by law to make them.",
            ["The statement is made before a person authorized by law to permit it to be made, and the person making it knows it is false."],
            "Subsection (1) does not apply to the stated kind of statement made during a criminal investigation.",
            "134", ["Summary conviction only."], SUMMARY, ["134", "787"]),
        "136" => guide!(
            "This offence concerns a witness giving contrary evidence in judicial proceedings.",
            ["The person first gives evidence about a matter of fact or knowledge as a witness, then gives contrary evidence in a later judicial proceeding.", "The court must be satisfied beyond a reasonable doubt that the person intended to mislead when giving evidence in one of the proceedings."],
            "The section applies whether or not either account is true, excludes evidence that is not material, and requires the Attorney General's consent before proceedings are instituted. It also addresses specified remote and international evidence.",
            "136", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["136"]),
        "137" => guide!(
            "This offence concerns fabricating something to be used as evidence in an existing or proposed judicial proceeding.",
            ["The person acts with intent to mislead and with intent that the fabricated item be used as evidence.", "The method is something other than perjury or incitement to perjury."],
            "",
            "137", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["137"]),
        "138" => guide!(
            "This section covers specified misconduct involving affidavits or statutory declarations.",
            ["Signing as the person taking an oath or declaration when it was not taken, or while knowing one lacks authority to administer it.", "Using or offering a purported affidavit or declaration while knowing it was not properly sworn or declared as described in the section.", "Signing as the person making a purported sworn statement when it was not actually sworn or declared."],
            "",
            "138", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["138", "787"]),
        "811" => guide!(
            "This offence concerns breaching a recognizance under one of the listed peace-bond provisions.",
            ["The person is bound by a recognizance under section 83.3 or sections 810 to 810.2.", "The person breaches that recognizance."],
            "A recognizance is a court-enforceable promise with conditions. Section 811 applies to the listed provisions; it does not describe every breach of a court order.",
            "811", ["Up to 4 years on indictment, or summary conviction."], SUMMARY, ["811", "787"]),
        "810.4" => guide!(
            "This offence concerns misuse of bodily substances or test results provided under certain recognizances.",
            ["Using a bodily substance provided under a listed recognizance for a purpose other than determining compliance with its specified abstinence condition; or", "Using, disclosing or allowing disclosure of its analysis results outside the statutory exceptions."],
            "Section 810.4 lists the recognizances and permits specified disclosure to the defendant, use or disclosure in an investigation or proceeding for a section 811 offence, and anonymous statistical or other research.",
            "810.4", ["Summary conviction only."], SUMMARY, ["810.4", "811", "787"]),
        "83.03(1)" => guide!(
            "This offence concerns providing property or financial or related services for terrorist purposes.",
            ["A person directly or indirectly, wilfully and without lawful justification or excuse collects property, or provides, invites someone to provide, or makes available property or financial or related services.", "The person intends or knows that they will be used, wholly or partly, to facilitate or carry out terrorist activity, or to benefit a person facilitating or carrying it out."],
            "Subsection 83.03(4) excludes acts done solely for humanitarian assistance under the auspices of impartial humanitarian organizations, in accordance with international law, while using reasonable efforts to minimize benefits to terrorist groups. Its conditions matter.",
            "83.03", ["Indictable offence: up to 10 years' imprisonment."], INDICTABLE, ["83.03", "83.01"]),
        "83.03(2)" => guide!(
            "This offence concerns providing property or financial or related services for use by or to benefit a terrorist group.",
            ["A person directly or indirectly, wilfully and without lawful justification or excuse collects property, or provides, invites someone to provide, or makes available property or financial or related services.", "The person knows that they will be used by or benefit a terrorist group, wholly or partly."],
            "Subsection (3) excludes acts carried out under and in accordance with an authorization under section 83.032. Subsection (4) also excludes qualifying humanitarian assistance subject to its specific conditions.",
            "83.03", ["Indictable offence: up to 10 years' imprisonment."], INDICTABLE, ["83.03", "83.032", "83.01"]),
        "83.04" => guide!(
            "This offence concerns using or possessing property for terrorist purposes.",
            ["Using property, directly or indirectly and wholly or partly, to facilitate or carry out terrorist activity; or", "Possessing property intending or knowing it will be used for that purpose."],
            "Section 83.01 defines terms used in this part of the Code. The purpose and the alleged knowledge or intention matter.",
            "83.04", ["Indictable offence: up to 10 years' imprisonment."], INDICTABLE, ["83.04", "83.01"]),
        "83.12" => guide!(
            "This offence covers contraventions of the property restrictions, disclosure duties or continuing checks in sections 83.08, 83.1 and 83.11.",
            ["Section 83.08 prohibits specified knowing dealings, transactions and services involving property owned or controlled by or on behalf of a terrorist group; its geographic and other conditions apply.", "Section 83.1 requires the specified disclosures without delay concerning property in the person's possession or control that they know is owned or controlled by or on behalf of a terrorist group, and related transaction information.", "Section 83.11 requires the listed types of entities to conduct continuing checks and make reports, subject to its terms and regulations."],
            "Each underlying section has distinct conditions. The full provisions identify who is covered, the property and activities involved, and exceptions or protections.",
            "83.12", ["On indictment: up to 10 years' imprisonment.", "On summary conviction: a fine up to $100,000, imprisonment up to two years less a day, or both."],
            "Section 83.12 sets its own summary penalty. The general $5,000 summary fine limit is not the maximum stated in this section.", ["83.12", "83.08", "83.1", "83.11"]),
        "83.18" => guide!(
            "This offence concerns knowingly participating in or contributing to an activity of a terrorist group, directly or indirectly.",
            ["The purpose must be to enhance the ability of any terrorist group to facilitate or carry out terrorist activity.", "The section includes specified training, skills, recruitment, travel or remaining in a country, and making oneself available on a group's instructions."],
            "Subsection (2) addresses circumstances that need not be proven, including that an activity occurred, that the contribution actually enhanced the group's ability, or that the accused knew its specific nature. Subsection (4) lists factors a court may consider; those factors alone are not the whole offence.",
            "83.18", ["Indictable offence: up to 10 years' imprisonment."], INDICTABLE, ["83.18", "83.01"]),
        "83.181" => guide!(
            "This offence concerns leaving or attempting to leave Canada for specified terrorist-group participation abroad.",
            ["It also covers going or attempting to go aboard a conveyance intending to leave Canada.", "The purpose must be to commit an act or omission outside Canada that would be an offence under section 83.18(1) if committed in Canada."],
            "",
            "83.181", ["Indictable offence: up to 10 years' imprisonment."], INDICTABLE, ["83.181", "83.18"]),
        "83.19" => guide!(
            "This offence concerns knowingly facilitating a terrorist activity.",
            ["Facilitation can occur whether or not the facilitator knows the particular activity, a particular activity was foreseen or planned, or an activity was carried out."],
            "This rule must be read together with the offence's knowledge requirement.",
            "83.19", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["83.19", "83.01"]),
        "83.191" => guide!(
            "This offence concerns leaving or attempting to leave Canada to facilitate terrorist activity abroad.",
            ["It also covers going or attempting to go aboard a conveyance intending to leave Canada.", "The purpose must be to commit an act or omission outside Canada that would be an offence under section 83.19(1) if committed in Canada."],
            "Read the travel provision together with the related facilitation offence.",
            "83.191", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["83.191", "83.19"]),
        "83.2" => guide!(
            "This offence concerns committing an indictable offence for a terrorist group.",
            ["The underlying offence must be indictable under the Criminal Code or another federal Act.", "It must be committed for the benefit of, at the direction of, or in association with a terrorist group."],
            "",
            "83.2", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["83.2", "83.01"]),
        "83.201" => guide!(
            "This offence concerns leaving or attempting to leave Canada to commit an offence abroad for a terrorist group.",
            ["It also covers going or attempting to go aboard a conveyance intending to leave Canada.", "The purpose must be an act or omission abroad that would be a federal indictable offence if committed in Canada, for the benefit of, at the direction of, or in association with a terrorist group."],
            "",
            "83.201", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["83.201", "83.01"]),
        "83.202" => guide!(
            "This offence concerns leaving or attempting to leave Canada to commit a terrorist offence abroad.",
            ["It also covers going or attempting to go aboard a conveyance intending to leave Canada.", "The purpose must be an act or omission abroad that would be a federal indictable offence if committed in Canada, where the act or omission also constitutes terrorist activity."],
            "The travel purpose, underlying offence and statutory definition of terrorist activity matter.",
            "83.202", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["83.202", "83.01"]),
        "56.1" => guide!(
            "This offence concerns another person's identity document.",
            ["Without lawful excuse, a person has such a document made, possesses, transfers, sells or offers it for sale.", "The document relates, or claims to relate, wholly or partly to another person."],
            "Identity documents include specified government-issued documents such as passports, driving licences, health cards, social insurance number cards, and birth or death certificates. The section identifies permitted purposes, including specified good-faith work, genealogy, consent and legitimate administration of justice.",
            "56.1", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["56.1", "787"]),
        "57(1)" => guide!(
            "This offence concerns forging a passport or knowingly dealing with a forged passport as genuine.",
            ["The conduct can be forging the passport; using, dealing with or acting on it knowing it is forged; or causing or attempting to cause another person to do so as if genuine."],
            "The subsection applies inside or outside Canada. For proceedings under the section, where the passport was forged is not material.",
            "57", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["57"]),
        "57(2)" => guide!(
            "A false statement in relation to a passport involves the following:",
            ["A person makes an oral or written statement they know is false or misleading.", "The purpose is to obtain a passport for anyone, or a material alteration or addition to one."],
            "The subsection applies inside or outside Canada.", "57", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["57", "787"]),
        "57(3)" => guide!(
            "This offence concerns possessing a forged passport or a passport in respect of which the false-statement offence in subsection 57(2) was committed.",
            ["Possession must be without lawful excuse."], "The definition of passport and the applicable rules are in section 57.",
            "57", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["57", "787"]),
        "58" => guide!(
            "This offence concerns fraudulent use of a citizenship or naturalization certificate.",
            ["Using such a certificate for a fraudulent purpose; or", "Being the person to whom it was granted and knowingly parting with it intending that it be used for a fraudulent purpose."],
            "The section applies inside or outside Canada and defines the certificates by reference to the Citizenship Act.",
            "58", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["58", "787"]),
        "61" => guide!(
            "Section 61 covers speaking seditious words, publishing a seditious libel, or joining a seditious conspiracy.",
            ["Sections 59 and 60 describe the relevant terms, a presumption concerning advocacy of unlawful force for government change, and specified good-faith purposes."],
            "The meaning and constitutional application of these provisions depend on case law; good-faith purposes listed in section 60 are not, by themselves, seditious intention.",
            "61", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["61", "59", "60"]),
        "62" => guide!(
            "This offence covers specified intentional conduct affecting a member of a force.",
            ["Interfering with, impairing or influencing loyalty or discipline;", "Publishing or distributing writing that urges insubordination, disloyalty, mutiny or refusal of duty; or", "Advising, urging or causing such conduct."],
            "The section defines a member of a force to include Canadian Forces members and members of specified foreign forces lawfully in Canada.",
            "62", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["62", "787"]),
        "65" => guide!(
            "This offence concerns taking part in a riot.",
            ["A riot is an unlawful assembly that has begun to disturb the peace tumultuously.", "An unlawful assembly involves at least three people and the common purpose and reasonable-fear conditions in section 63."],
            "Section 63 also addresses a lawful assembly becoming unlawful and the specified dwelling-house exception.",
            "65", ["Basic offence: up to 2 years on indictment, or summary conviction.", "Wearing a mask or disguise to conceal identity without lawful excuse while committing it: up to 10 years on indictment, or summary conviction."], SUMMARY, ["65", "63", "64", "787"]),
        "66" => guide!(
            "This offence concerns membership in an unlawful assembly.",
            ["The definition involves at least three people with a common purpose and the conduct and reasonable-fear conditions in section 63, including needless provocation of others."],
            "A lawful assembly can become unlawful in the circumstances in section 63. The section also contains a specific dwelling-house protection rule.",
            "66", ["Basic offence: summary conviction.", "Wearing a mask or disguise to conceal identity without lawful excuse while committing it: up to 5 years on indictment, or summary conviction."], SUMMARY, ["66", "63", "787", "737"]),
        "68" => guide!(
            "This section concerns offences related to a proclamation to disperse a riot.",
            ["Wilfully using force to oppose, hinder or assault a person making or about to make the proclamation so it is not made;", "Failing to disperse and depart peaceably within 30 minutes after it is made; or", "Failing to depart within 30 minutes when the section's reasonable-belief conditions about a prevented proclamation apply."],
            "Section 67 describes who makes the proclamation and the circumstances involving twelve or more persons unlawfully and riotously assembled.",
            "68", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["68", "67"]),
        "69" => guide!(
            "This offence applies to a peace officer notified of a riot within their jurisdiction.",
            ["Without reasonable excuse, the officer fails to take all reasonable steps to suppress the riot."],
            "",
            "69", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["69", "787"]),
        "70(3)" => guide!(
            "This offence concerns contravening an order made under section 70.",
            ["The Governor in Council may make the kinds of proclamation orders described in the section concerning drilling, training or military exercises."],
            "An order may be general or apply to specified places, districts or assemblies; its actual terms matter.",
            "70", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["70", "787"]),
        "72(1)" => guide!(
            "Forcible entry concerns entering real property in another person's actual and peaceable possession.",
            ["The manner of entry is likely to cause a breach of the peace or a reasonable apprehension of one."],
            "Section 72 addresses entitlement to enter and intent to possess. Section 73 sets the punishment.",
            "73", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["72", "73", "787"]),
        "72(2)" => guide!(
            "Forcible detainer concerns holding real property against a person legally entitled to possession.",
            ["The person holding it is in actual possession without colour of right.", "The manner of holding it is likely to cause a breach of the peace or a reasonable apprehension of one."],
            "Section 72 addresses possession and colour of right. Section 73 sets the punishment.",
            "73", ["Up to 2 years on indictment, or summary conviction."], SUMMARY, ["72", "73", "787"]),
        "74" => guide!(
            "This offence concerns an act that is piracy under the law of nations.",
            ["The section does not list every qualifying act."],
            "The punishment provision applies to piracy committed inside or outside Canada.",
            "74", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["74"]),
        "75" => guide!(
            "Piratical acts cover specified conduct involving a Canadian ship, inside or outside Canada.",
            ["Stealing the ship; stealing cargo, supplies or fittings, or throwing them overboard, damaging or destroying them without lawful authority;", "Doing or attempting a mutinous act on the ship; or counselling someone to do any of those acts."],
            "",
            "75", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["75"]),
        "76" => guide!(
            "Hijacking concerns unlawfully seizing or exercising control of an aircraft by force, threat of force or other intimidation.",
            ["The person must have one of the specified intentions, including confining someone aboard, taking them to a place other than the next scheduled landing, holding them for ransom or service, or causing a material deviation from the flight plan."],
            "",
            "76", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["76"]),
        "77" => guide!(
            "This section covers several acts endangering aircraft or airport safety.",
            ["Examples include qualifying violence aboard an aircraft or at an international airport; damaging an aircraft, placing a dangerous item aboard, or interfering with navigation facilities;", "It also addresses specified attacks on international airport facilities or services and knowingly communicating false information that endangers an aircraft in flight."],
            "Each alternative has particular conditions, including the required danger and, for some airport acts, use of a weapon, substance or device.",
            "77", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["77"]),
        "78" => guide!(
            "This offence concerns taking an offensive weapon or explosive substance aboard a civil aircraft.",
            ["The person is not a peace officer executing their duty.", "They act without the specified consent, or with consent but without complying with all its terms and conditions."],
            "The section defines civil aircraft and excludes specified military, police and enforcement aircraft.",
            "78", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["78"]),
        "78.1" => guide!(
            "This section sets out offences involving ships and fixed platforms.",
            ["Seizing or controlling one by force, threat of force or other intimidation;", "Specified violence, damage, interference with navigation facilities, or placement of a damaging item that is likely to endanger safe navigation or platform safety;", "Knowingly communicating false information that endangers safe navigation; or making certain threats to compel action where the threat is likely to endanger safety."],
            "The section defines ship and fixed platform and excludes specified vessels. Each alternative has its own conditions.",
            "78.1", ["Each listed offence is indictable and carries up to life imprisonment."], INDICTABLE, ["78.1"]),
        "80" => guide!(
            "Section 79 imposes a duty of reasonable care on a person who possesses or controls an explosive substance.",
            ["Section 80 addresses failure to perform that duty without lawful excuse.", "If an explosion results, the penalty provisions distinguish death or likely death from bodily harm, property damage or their likelihood."],
            "The offence wording and the conditions attached to the specified penalties should be read together in sections 79 and 80.",
            "80", ["Indictable offence: where an ensuing explosion causes or is likely to cause death, up to life imprisonment; where it causes or is likely to cause bodily harm or property damage, up to 14 years."], INDICTABLE, ["80", "79"]),
        "81" => guide!(
            "This section covers several prohibited acts involving explosives or other dangerous substances.",
            ["Acting with intent to cause an explosion likely to cause serious bodily harm, death or serious property damage;", "With intent to cause bodily harm, causing an explosion or sending, delivering, placing or throwing specified dangerous substances or things;", "Placing or throwing an explosive with intent to damage property without lawful excuse; or making or controlling an explosive with specified intent to endanger life or seriously damage property."],
            "",
            "81", ["Indictable: up to life for paragraphs (1)(a) or (b); up to 14 years for paragraphs (1)(c) or (d)."], INDICTABLE, ["81"]),
        "82(1)" => guide!(
            "This offence applies to conduct without lawful excuse:",
            ["Making or possessing an explosive substance, or having one under care or control."], "Subsection 82(2) separately addresses conduct associated with a criminal organization.",
            "82", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["82", "787"]),
        "82(2)" => guide!(
            "This offence concerns making or possessing an explosive substance, or having one under care or control, without lawful excuse in connection with a criminal organization.",
            ["The conduct must be for the organization's benefit, at its direction or in association with it."],
            "Section 82.1 sets out a consecutive-sentence rule for this offence.",
            "82", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["82", "82.1"]),
        "82.3" => guide!(
            "This offence concerns specified acts involving nuclear or radioactive material, a device or a nuclear facility.",
            ["The person intends to cause death, serious bodily harm, or substantial damage to property or the environment.", "The prohibited acts include making a device, possessing or dealing with the specified material or device, or specified acts against or serious interference with a nuclear facility."],
            "",
            "82.3", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["82.3"]),
        "82.4" => guide!(
            "This offence concerns using or altering nuclear or radioactive material or a device, or specified acts affecting a nuclear facility.",
            ["The person intends to compel a person, government or international organization to do or refrain from an act."],
            "",
            "82.4", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["82.4"]),
        "82.5" => guide!(
            "This offence concerns committing an indictable offence under a federal Act with a specified purpose.",
            ["The person intends to obtain nuclear material, radioactive material or a device, or access to a nuclear facility."],
            "",
            "82.5", ["Indictable offence: up to life imprisonment."], INDICTABLE, ["82.5"]),
        "82.6" => guide!(
            "This offence concerns threats involving nuclear or radioactive material.",
            ["The threat must be to commit an offence under sections 82.3 to 82.5."],
            "",
            "82.6", ["Indictable offence: up to 14 years' imprisonment."], INDICTABLE, ["82.6", "82.3", "82.4", "82.5"]),
        "83" => guide!(
            "This offence concerns specified participation in a prize fight.",
            ["Fighting as a principal; advising, encouraging or promoting it; or attending in one of the listed support or reporting roles."],
            "The definition in section 83 excludes qualifying amateur contests and certain provincially authorized boxing and mixed martial arts contests; the conditions and permissions matter.",
            "83", ["Summary conviction."], SUMMARY, ["83", "787", "737"]),
        "83.02" => guide!(
            "This offence concerns wilfully providing or collecting property, directly or indirectly, without lawful justification or excuse for specified terrorist acts.",
            ["The person must intend or know that the property will be used, wholly or partly, for an act or omission described in section 83.02.", "The section covers specified offences referred to in the terrorist-activity definition and certain other acts intended to cause death or serious bodily harm with the stated intimidation or compulsion purpose."],
            "The definition in section 83.01 and the precise intention, knowledge and purpose must be considered.",
            "83.02", ["Indictable offence: up to 10 years' imprisonment."], INDICTABLE, ["83.02", "83.01"]),
        _ => None,
    }
}

fn with_statutory_minimum(mut guide: Option<ChargeGuide>, minimum: &'static str) -> Option<ChargeGuide> {
    if let Some(ref mut value) = guide { value.minimum = minimum; }
    guide
}

fn weapons_guide(number: &str) -> Option<ChargeGuide> {
    match number {
        "100" => with_statutory_minimum(guide!(
            "This offence concerns possessing specified firearms, weapons, devices, firearm parts or ammunition for an unauthorized transfer.",
            ["The purpose is to transfer the item or offer to transfer it, whether or not payment is involved.", "The person knows they are not authorized to transfer it under the applicable federal Acts or regulations."],
            "The items and the applicable penalty depend on the exact statutory category. Section 101 separately addresses an unauthorized transfer.",
            "100", ["Indictable offence: up to 14 years' imprisonment."],
            "The provision specifies minimum terms for the items identified in subsection (2). Their application can depend on constitutional and other case law.", ["100"]),
            "The statute specifies 3 years for a first offence and 5 years for a second or later offence involving a firearm, prohibited device, firearm part or ammunition listed in subsection (2). Subsection (3) specifies no minimum for the other cases."),
        "101" => guide!(
            "This offence concerns transferring specified firearms, weapons, devices, firearm parts or ammunition without the required federal authority.",
            ["The transfer is to another person and lacks the authority required under federal law."],
            "Unlike section 100, section 101 does not expressly include knowledge of the lack of authorization as an element. Applicable case law and any defence still matter.",
            "101", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["101", "787"]),
        "102" => with_statutory_minimum(guide!(
            "This offence concerns making an automatic firearm without lawful excuse.",
            ["The person alters a firearm so it can discharge projectiles in rapid succession during one trigger pressure, or manufactures or assembles a firearm capable of doing so."],
            "",
            "102", ["Up to 10 years on indictment, or summary conviction."],
            "The provision specifies a minimum on indictment. Its application can depend on constitutional and other case law. The general summary penalty is in section 787.", ["102", "787"]),
            "The statute specifies 1 year on indictment; subsection (2)(b) specifies no minimum for summary conviction."),
        "102.1" => guide!(
            "This section covers two offences involving computer data for firearms or prohibited devices.",
            ["Possessing or accessing data covered by subsection (1), capable of use with a 3D printer, metal milling machine or similar computer system for the specified unauthorized manufacturing or trafficking purpose.", "Distributing, publishing or making data available as described in subsection (2), knowing they are intended for the specified unauthorized manufacturing or trafficking purpose."],
            "The section excludes a firearm deemed not to be a firearm under subsection 84(3). The applicable authorization, data and required purpose or knowledge matter.",
            "102.1", ["Up to 10 years on indictment, or summary conviction."], SUMMARY, ["102.1", "84", "787"]),
        "103" => with_statutory_minimum(guide!(
            "This offence concerns importing or exporting specified weapons and related items knowing the activity is unauthorized.",
            ["The items include specified firearms, weapons, prohibited devices, firearm parts and prohibited ammunition, plus the exclusively designed automatic-firearm components described in the section.", "The person knows they are not authorized under the applicable federal Acts or regulations."],
            "Section 104 separately addresses unauthorized importing or exporting without the same express knowledge wording.",
            "103", ["Indictable offence: up to 14 years' imprisonment."],
            "The provision specifies minimum terms for the items identified in subsection (2). Their application can depend on constitutional and other case law.", ["103"]),
            "The statute specifies 3 years for a first offence and 5 years for a second or later offence involving a firearm, prohibited device, firearm part or prohibited ammunition listed in subsection (2). Subsection (2.1) specifies no minimum for the other cases."),
        "104" => guide!(
            "This offence concerns importing or exporting specified weapons and related items without federal authority.",
            ["The items include specified firearms, weapons, prohibited devices, firearm parts, prohibited ammunition and the exclusively designed automatic-firearm components described in the section."],
            "Unlike section 103, section 104 does not expressly include knowledge of the lack of authorization as an element. Applicable case law and any defence still matter.",
            "104", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["104", "787"]),
        "104.1" => guide!(
            "This offence concerns altering a cartridge magazine without lawful excuse.",
            ["The magazine was not a prohibited device before the alteration, and the alteration makes it one."],
            "",
            "104.1", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["104.1", "84", "787"]),
        "105" => guide!(
            "This section concerns reporting specified lost, stolen or found items.",
            ["A person who loses, or has stolen from their possession, an item listed in subsection (1)(a) must report the loss with reasonable dispatch to a peace officer, firearms officer or chief firearms officer.", "A person who finds an item listed in subsection (1)(b), with reasonable grounds to believe it was lost or abandoned, must deliver it or report the finding with reasonable dispatch to one of those officers."],
            "The lists are not identical: authorizations, licences and registration certificates appear in the loss provision, while the finding provision names specified weapons, devices and prohibited ammunition.",
            "105", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["105", "787"]),
        "106" => guide!(
            "This offence concerns failing to report the destruction of certain firearms, weapons, devices or prohibited ammunition.",
            ["After destroying a listed item, or learning that a listed item previously in their possession was destroyed, the person does not report the destruction with reasonable dispatch to a peace officer, firearms officer or chief firearms officer."],
            "The section lists prohibited and restricted firearms and other specified items; it does not list non-restricted firearms.",
            "106", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["106", "787"]),
        "107" => guide!(
            "This offence concerns knowingly making a false report or statement to a peace officer, firearms officer or chief firearms officer.",
            ["The report or statement concerns the loss, theft or destruction of an item listed in section 107, including specified firearms, weapons, devices, prohibited ammunition, authorizations, licences or registration certificates."],
            "The section defines a report or statement to include an assertion of fact, opinion, belief or knowledge, whether material or admissible or not.",
            "107", ["Up to 5 years on indictment, or summary conviction."], SUMMARY, ["107", "787"]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_pasted_entries_have_unique_guides_and_sourced_sections() {
        let entries = ["56.1", "57(1)", "57(2)", "57(3)", "58", "61", "62", "65", "66", "68",
            "69", "70(3)", "72(1)", "72(2)", "74", "75", "76", "77", "78", "78.1",
            "80", "81", "82(1)", "82(2)", "82.3", "82.4", "82.5", "82.6", "83", "83.02",
            "83.03(1)", "83.03(2)", "83.04", "83.12", "83.18", "83.181", "83.19", "83.191", "83.2", "83.201", "83.202", "810.4", "811",
            "126", "127", "128", "129", "130", "132", "134", "136", "137", "138"];
        for number in entries {
            let matches: Vec<_> = super::super::CHARGES.iter()
                .filter(|c| c.category == Category::PublicOrder && c.number == number).collect();
            assert_eq!(matches.len(), 1, "missing or duplicate entry {number}");
            let guide = matches[0].guide().expect("entry must have a guide");
            assert!(!guide.introduction.is_empty() && !guide.points.is_empty());
            assert!(!guide.minimum.is_empty() && !guide.penalties.is_empty());
            let primary = number.split('(').next().unwrap();
            assert!(guide.law_sections.contains(&primary), "missing primary section for {number}");
            assert!(guide.law_sections.contains(&guide.penalty_section), "missing punishment section for {number}");
            for section in guide.law_sections {
                assert!(crate::statutes::lookup("C-46", section).is_some(), "missing source {section} for {number}");
            }
        }
    }

    #[test]
    fn weapons_entries_are_unique_and_have_statutory_sources() {
        for number in ["100", "101", "102", "102.1", "103", "104", "104.1", "105", "106", "107"] {
            let matches: Vec<_> = super::super::CHARGES.iter()
                .filter(|c| c.category == Category::Weapons && c.number == number).collect();
            assert_eq!(matches.len(), 1, "missing or duplicate entry {number}");
            let guide = matches[0].guide().expect("weapons entry must have a guide");
            assert!(!guide.introduction.is_empty() && !guide.points.is_empty() && !guide.penalties.is_empty());
            assert!(guide.law_sections.contains(&number));
            for section in guide.law_sections {
                assert!(crate::statutes::lookup("C-46", section).is_some(), "missing source {section} for {number}");
            }
        }
        for number in ["100", "102", "103"] {
            let guide = weapons_guide(number).unwrap();
            assert!(guide.minimum.contains("statute"), "minimum should be identified as statutory");
        }
    }
}
