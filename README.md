# Software Engineering Excellence: MVP for a geo-distance, weighted ranking algorithm

**Table of contents**
<!-- TOC -->
* [The Challenge](#the-challenge)
  * [Task](#task)
  * [Functional Requirements](#functional-requirements)
    * [User Interface](#--user-interface)
    * [API Endpoints](#--api-endpoints)
  * [User Flows](#user-flow)
  * [How to Rank](#how-to-rank)
    * [Result List Criteria](#result-list-criteria)
    * [Adapting to Different Postcode Scenarios](#adapting-to-different-postcode-scenarios)
    * [Business Rationale](#business-rationale)
    * [Calculate RANK](#calculate-rank)
    * [Filter By MAX_DRIVING_DISTANCE](#filter-by-maxdrivingdistance)
    * [Dynamic Rank Calculation: Adaptability to Data Changes](#dynamic-rank-calculation-adaptability-to-data-changes)
* [Instructions](#instructions)
  * [How to Deliver Your Solution](#how-to-deliver-your-solution)
  * [How We Test & Evaluate Your Solution](#how-we-test--evaluate-your-solution)
    * [UX/UI part](#uxui-part)
    * [Code quality](#code-quality)
    * [Performance](#performance)
* [Resources](#resources)
  * [Data Sets](#data-sets)
  * [Useful Links](#useful-links)
<!-- TOC -->

## The Challenge

### Task

Develop an MVP (Minimal Viable Product) for the CHECK24 Craftsmen Comparison (Handwerkervergleich). 

Your MVP will encompass three primary components: a frontend, a backend, and a persistence layer. 
The core objective is to design a frontend interface where users can input their postal code to receive a ranked list of craftsmen. The ranking system should incorporate a weighted rank based on the distance between the craftsman and the user, as well as the craftsman's quality factor. These weights should dynamically adjust depending on the user's postal code. 

You will be granted access to a comprehensive, anonymized dataset of craftsmen, available in both SQL and JSON formats.

Performance is key! Your solution will be assessed not just on functionality but also on the speed of generating the craftsmen list. A swift response time is critical for an exceptional user experience.

You're free to choose any technology stack, but the emphasis of this challenge lies in software engineering prowess. This entails selecting the optimal tools and strategies to seamlessly integrate them for a superior solution.

### Functional Requirements

#### User Interface

Your MVP must meet the following requirements. While some are essential, others marked as 'optional' can enhance the jury's assessment of your deliverable:

- Postcode input page
- Result list page
  - Shows `n=20` top ranked service providers
  - "Load More" button to load the next 20 service providers
  - *[Optional]* Sort by `DISTANCE` or `PROFILE_SCORE` only
- Updating `MAX_DRIVING_DISTANCE`, `PROFILE_PICTURE_SCORE` and  `PROFILE_DESCRIPTION_SCORE`
  - *[Optional]* Provide UI to update the variables

#### API Endpoints

Additionally please make sure your app also expose following endpoints which we can use for performance test later:

* `GET /craftsmen?postalcode={postalcode}`: Return up to the top 20 ranked service providers for a given postalcode(or fewer if there are less than 20 available).

  response model:

  ```typescript
  interface Craftsman {
      id: number;
      name: string; // firstname + lastname
      rankingScore: number;
  }
  
  interface Response {
      craftsmen: Craftsman[];
  }
  ```
  
* `PATCH /craftman/{craftman_id}`: updating `MAX_DRIVING_DISTANCE`, `PROFILE_PICTURE_SCORE` and  `PROFILE_DESCRIPTION_SCORE` for a given service provider.

  request body

  ```typescript
  interface PatchRequest {
    // At least one of the attributes should be defined
    maxDrivingDistance?: number;
    profilePictureScore?: number;
    profileDescriptionScore?: number;
  }
  ```
  
  response model
  
  ```typescript
  interface PatchResponse {
    id: number;
    updated: {
      maxDrivingDistance: number;
      profilePictureScore: number;
      profileDescriptionScore: number;
    }
  }
  ```
  



We also provide a OpenAPI spec for the two endpoints above. Please check `endpoints.yaml` file in this project.

### User Flow

To provide you with a clear vision of what your MVP could resemble, we have shared a series of screenshots from our live production website,
[handwerk.check24.de](https://handwerk.check24.de/craftsmen/handwerker?deviceoutput=mobile) to give you a good impression how your MVP might look like.

#### Step 1
The customer begins their journey by entering a postcode, specifying the geographical area where they are seeking the services of craftsmen. This initial step is crucial as it sets the foundation for the tailored, location-based results that the system will generate.

<img src="screen01.png" width="250px" />

#### Step 2
The customer is then shown a ranked list of craftsmen, organized based on the algorithm that considers distance and quality factors, as per the user's postcode input.

<img src="screen02.png" width="250px" />

#### Step 3
As the customer scrolls to the bottom of the list, a 'Load More' button appears, allowing them to access additional craftsmen from the extended list.

<img src="screen03.png" width="250px" />

### How to Rank
For the "Handwerker" vertical, CHECK24 has established a specific set of rules to curate a well-organized and relevant list of craftsman service providers.

#### Result List Criteria

Your task is to generate a result list that prioritizes the top-ranked service providers based on a given postcode. Initially, the list should display a default number of `n = 20` service providers.

Customers have the option to explore beyond the initial set by scrolling or paging through the list, thus loading more than the original n service providers.

#### Adapting to Different Postcode Scenarios

It's important to consider the varying density of service providers in different postcode areas. This density is categorized into distance groups, influencing whether an area is classified as having a high (`group_a`), medium (`group_b`) or low (`group_c`) concentration of service providers.

#### Business Rationale

Our aim is to ensure excellent service provider availability across all areas, including those with lower densities. To achieve this, we adjust the calculation of the driving distance, extending the range service providers are considered available to travel. This approach ensures a balanced and fair representation of service providers, enhancing customer experience even in less densely populated areas.

Based on the group, we calculate the service provider's individual maximum driving distance like this:

```
IF group_a THEN
  MAX_DRIVING_DISTANCE = MAX_DRIVING_DISTANCE + 0
ELSE IF group_b THEN
  MAX_DRIVING_DISTANCE = MAX_DRIVING_DISTANCE + 2km
ELSE IF group_c THEN
  MAX_DRIVING_DISTANCE = MAX_DRIVING_DISTANCE + 5km
END IF
```

#### Calculate RANK
The dynamic `RANK` of each service provider is calculated as followed:

```
PROFILE_SCORE = 0.4*PROFILE_PICTURE_SCORE + 0.6*PROFILE_DESCRIPTION_SCORE

DISTANCE = DISTANCE BETWEEN (POSTCODE_LON, POSTCODE_LAT) AND (SERVICE_PROVIDER_LON, SERVICE_PROVIDER_LAT)

DEFAULT_DISTANCE = 80
DISTANCE_SCORE = 1 - (DISTANCE/DEFAULT_DISTANCE)
DISTANCE_WEIGHT = DISTANCE > DEFAULT_DISTANCE ? 0.01 : 0.15

RANK = DISTANCE_WEIGHT*DISTANCE_SCORE + (1 - DISTANCE_WEIGHT)*PROFILE_SCORE
```

#### Filter By MAX_DRIVING_DISTANCE
Only those service providers should be ranked whose individual adapted `MAX_DRIVING_DISTANCE` is higher than the actual calculated `DISTANCE`.


#### Dynamic Rank Calculation: Adaptability to Data Changes

It's crucial to design your ranking algorithm with flexibility in mind. The datasets provided are initial references, but remember, the actual data can shift at any moment. Your solution should be robust enough to accommodate these changes. Here are ideas how you could approach this:

- Asynchronous Updates: The ranking algorithm doesn't need to reflect data changes instantly. It’s acceptable if the ranks are updated in an asynchronous manner. This approach allows for periodic recalculations, ensuring your algorithm stays current without the need for real-time data adjustments.
- Algorithm Flexibility: Design your algorithm to be adaptable. It should efficiently handle varying data volumes and content changes without a complete overhaul. This might involve modular design principles or incorporating features that allow easy parameter adjustments.
- Regular Data Refreshes: Implement a system for regularly updating the dataset your algorithm uses. This could be through scheduled data refreshes or triggers based on specific conditions.
- Maintaining Performance: Despite these dynamic updates, ensure that the performance and speed of the ranking process are not compromised.

## Instructions

### How to Deliver Your Solution

* The project should be submitted to [DevPost](https://devpost.com/) no later than 10 AM on November 19, 2023. (please check the latest information on discord)
* In the submission, you should also provide us a GitHub repository link where we can clone your project.
  * It would be good/suggested that you can share your repo(via discord#check24 channel) already on the first day of the event and do regularly commit & push, so that we can have more time to evaluate your code quality.

* We expect to run `docker-compose up` to start your app.

### How We Test & Evaluate Your Solution

We will evaluate your solution from three aspects:

#### UX/UI part
Make sure your application fulfills standards of modern web applications, like:
* Responsive Design
* Look & Feel
* PageSpeed
* ...

#### Code quality
* Loose coupling & high cohesion
* Clear structure
* Clean code
* ...

#### Performance
* Please make sure your app also exposes two endpoints which described in the ["Functional Requirements"](#functional-requirements) where we can:
  * Querying up to top 20 ranked service providers by a given postalcode.
  * Updating `MAX_DRIVING_DISTANCE`, `PROFILE_PICTURE_SCORE` and  `PROFILE_DESCRIPTION_SCORE` for a specific service provider.
* We will run some test cases against these two endpoints and for each test case:
  * We will check whether the response is correct.
  * We will measure the response time of **correct responses only**.


## Resources

### Data Sets
- Service Providers:
  - `service_provider_profile.sql`
  - `service_provider_profile.json`

- Quality Factor Scores:
  - `quality_factor_score.sql`
  - `quality_factor_score.json`

- Postcodes:
  - `postcode.sql`
  - `postcode.json`

### Useful Links
Calculate distance between coordinates in meters:
http://janmatuschek.de/LatitudeLongitudeBoundingCoordinates

